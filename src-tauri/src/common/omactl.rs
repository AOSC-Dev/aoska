use crate::common::omactl_types::{
    OmactlCapabilitiesPayload, OmactlEnvelope, OmactlErrorPayload, OmactlUnitPayload,
    PmCapabilities, PmOperationStart, PmUpdateSummary, TumUpdateInfo, SCHEMA_VERSION,
};
use anyhow::{anyhow, bail, Context, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("oma is busy {0}")]
pub struct OmaBusyError(String);

impl OmaBusyError {
    pub fn new(unit: Option<&str>) -> Self {
        let s = unit.map(|u| format!(" (unit={u})")).unwrap_or_default();
        Self(s)
    }
}

#[derive(Error, Debug)]
pub enum OmactlJsonError {
    #[error("omactl returned schema_version={actual}, expected {expected} for kind {kind}")]
    UnsupportedSchema {
        actual: u64,
        expected: u64,
        kind: String,
    },
    #[error("omactl returned kind={actual}, expected one of {expected:?}")]
    UnexpectedKind {
        actual: String,
        expected: Vec<String>,
    },
    #[error("omactl error {code}: {message}{details}")]
    CommandError {
        code: String,
        message: String,
        details: String,
    },
    #[error("omactl error response for kind {kind} did not include structured error payload")]
    MissingError { kind: String },
    #[error("omactl success response for kind {kind} did not include data")]
    MissingData { kind: String },
}

fn run_omactl(args: &[String]) -> Result<String> {
    let out = Command::new("omactl")
        .args(args)
        .output()
        .context("failed to spawn omactl")?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    if !stdout.trim().is_empty() {
        return Ok(stdout);
    }
    if out.status.success() {
        return Ok(stdout);
    }
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    bail!(
        "omactl failed without JSON: status={:?} stderr={}",
        out.status.code(),
        stderr
    )
}

fn parse_json<T>(stdout: &str, expected_kinds: &[&str]) -> Result<T>
where
    T: DeserializeOwned,
{
    let envelope: OmactlEnvelope =
        serde_json::from_str(stdout).context("invalid omactl JSON envelope")?;
    if envelope.schema_version != SCHEMA_VERSION {
        return Err(OmactlJsonError::UnsupportedSchema {
            actual: envelope.schema_version,
            expected: SCHEMA_VERSION,
            kind: envelope.kind,
        }
        .into());
    }

    if !envelope.ok {
        return Err(omactl_error(envelope.kind, envelope.error).into());
    }

    let accepted = accepted_kinds(expected_kinds);
    if !accepted.iter().any(|k| k == &envelope.kind) {
        return Err(OmactlJsonError::UnexpectedKind {
            actual: envelope.kind,
            expected: accepted,
        }
        .into());
    }

    let kind = envelope.kind;
    let data = envelope.data.ok_or(OmactlJsonError::MissingData { kind })?;
    serde_json::from_value(data).context("invalid omactl JSON payload")
}

fn accepted_kinds(expected_kinds: &[&str]) -> Vec<String> {
    expected_kinds
        .iter()
        .flat_map(|kind| [kind.to_string(), format!("{kind}.v1")])
        .collect()
}

fn omactl_error(kind: String, error: Option<OmactlErrorPayload>) -> OmactlJsonError {
    match error {
        Some(error) => OmactlJsonError::CommandError {
            code: error.code,
            message: error.message,
            details: error
                .details
                .map(|details| format!(" details={details}"))
                .unwrap_or_default(),
        },
        None => OmactlJsonError::MissingError { kind },
    }
}

fn json_value(args: &[String], expected_kinds: &[&str]) -> Result<Value> {
    let stdout = run_omactl(args)?;
    parse_json(&stdout, expected_kinds)
}

fn json_payload<T>(args: &[String], expected_kinds: &[&str]) -> Result<T>
where
    T: DeserializeOwned,
{
    let stdout = run_omactl(args)?;
    parse_json(&stdout, expected_kinds)
}

fn parse_exact_json<T>(stdout: &str, expected_kind: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let envelope: OmactlEnvelope =
        serde_json::from_str(stdout).context("invalid omactl JSON envelope")?;
    if envelope.schema_version != SCHEMA_VERSION {
        return Err(OmactlJsonError::UnsupportedSchema {
            actual: envelope.schema_version,
            expected: SCHEMA_VERSION,
            kind: envelope.kind,
        }
        .into());
    }
    if !envelope.ok {
        return Err(omactl_error(envelope.kind, envelope.error).into());
    }
    if envelope.kind != expected_kind {
        return Err(OmactlJsonError::UnexpectedKind {
            actual: envelope.kind,
            expected: vec![expected_kind.to_string()],
        }
        .into());
    }
    let kind = envelope.kind;
    let data = envelope.data.ok_or(OmactlJsonError::MissingData { kind })?;
    serde_json::from_value(data).context("invalid omactl JSON payload")
}

fn args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_string()).collect()
}

fn run_args(action: &str, packages: &[String], assume_yes: bool) -> Vec<String> {
    let mut args = vec!["run".to_string(), action.to_string(), "--json".to_string()];
    if assume_yes {
        args.push("--yes".to_string());
    }
    args.extend(packages.iter().cloned());
    args
}

fn push_unit(args: &mut Vec<String>, unit: Option<&str>) {
    if let Some(unit) = unit {
        args.push("--unit".to_string());
        args.push(unit.to_string());
    }
}

pub fn capabilities() -> Result<PmCapabilities> {
    let stdout = run_omactl(&args(&["capabilities", "--json"]))?;
    let raw: Value = parse_exact_json(&stdout, "omactl.capabilities")?;
    let payload: OmactlCapabilitiesPayload =
        serde_json::from_value(raw.clone()).context("invalid omactl capabilities payload")?;
    Ok(PmCapabilities {
        capabilities: payload.capabilities,
        raw,
    })
}

pub fn require_capability(capability: &str) -> Result<()> {
    let capabilities = capabilities()?;
    if capabilities
        .capabilities
        .iter()
        .any(|item| item == capability)
    {
        Ok(())
    } else {
        bail!("missing omactl capability: {capability}")
    }
}

pub fn query_installed() -> Result<Value> {
    json_value(
        &args(&["query", "installed", "--json"]),
        &["omactl.query.installed"],
    )
}

pub fn query_upgradable() -> Result<Value> {
    json_value(
        &args(&["query", "upgradable", "--json"]),
        &["omactl.query.upgradable"],
    )
}

pub fn query_package_detail(packages: &[String]) -> Result<Value> {
    if packages.is_empty() {
        bail!("packages is empty");
    }
    let mut args = args(&["query", "package-detail", "--json"]);
    args.extend(packages.iter().cloned());
    json_value(&args, &["omactl.query.package-detail"])
}

pub fn update_summary() -> Result<PmUpdateSummary> {
    let raw = query_upgradable()?;
    Ok(PmUpdateSummary {
        total: count_packages(&raw),
        security: None,
        security_classification_available: false,
        raw,
    })
}

pub fn query_tum_updates() -> Result<Vec<TumUpdateInfo>> {
    require_capability("plan.tum.v1").context("TUM_UNAVAILABLE: plan.tum.v1 is not advertised")?;
    require_capability("plan.upgrade.v1")
        .context("TUM_UNAVAILABLE: plan.upgrade.v1 is not advertised")?;
    let plan = plan_upgrade(&[])?;
    extract_tum_updates(&plan)
}

fn extract_tum_updates(plan: &Value) -> Result<Vec<TumUpdateInfo>> {
    if plan
        .pointer("/tum/available")
        .or_else(|| plan.get("tum_available"))
        .and_then(Value::as_bool)
        == Some(false)
    {
        bail!("TUM_UNAVAILABLE: omactl plan reports TUM data unavailable");
    }
    for pointer in [
        "/tum/updates",
        "/tum/groups",
        "/tum_updates",
        "/security_updates",
    ] {
        if let Some(items) = plan.pointer(pointer).and_then(Value::as_array) {
            return serde_json::from_value(Value::Array(items.clone()))
                .context("invalid TUM update group payload");
        }
    }
    bail!("TUM_UNAVAILABLE: omactl plan did not include TUM update groups")
}

pub fn plan_upgrade(packages: &[String]) -> Result<Value> {
    let mut args = args(&["plan", "upgrade", "--json"]);
    args.extend(packages.iter().cloned());
    json_value(&args, &["omactl.plan.upgrade"])
}

pub fn plan_install(packages: &[String]) -> Result<Value> {
    if packages.is_empty() {
        bail!("packages is empty");
    }
    let mut args = args(&["plan", "install", "--json"]);
    args.extend(packages.iter().cloned());
    json_value(&args, &["omactl.plan.install"])
}

pub fn plan_remove(packages: &[String]) -> Result<Value> {
    if packages.is_empty() {
        bail!("packages is empty");
    }
    let mut args = args(&["plan", "remove", "--json"]);
    args.extend(packages.iter().cloned());
    json_value(&args, &["omactl.plan.remove"])
}

pub fn run_upgrade(
    packages: &[String],
    assume_yes: bool,
    unit: Option<&str>,
) -> Result<PmOperationStart> {
    let mut args = run_args("upgrade", packages, assume_yes);
    push_unit(&mut args, unit);
    run_operation("upgrade", &args)
}

pub fn run_install(
    packages: &[String],
    assume_yes: bool,
    unit: Option<&str>,
) -> Result<PmOperationStart> {
    if packages.is_empty() {
        bail!("packages is empty");
    }
    let mut args = run_args("install", packages, assume_yes);
    push_unit(&mut args, unit);
    run_operation("install", &args)
}

pub fn run_remove(
    packages: &[String],
    remove_config: bool,
    assume_yes: bool,
    unit: Option<&str>,
) -> Result<PmOperationStart> {
    if packages.is_empty() {
        bail!("packages is empty");
    }
    let mut args = run_args("remove", packages, assume_yes);
    if remove_config {
        args.insert(3, "--remove-config".to_string());
    }
    push_unit(&mut args, unit);
    run_operation("remove", &args)
}

pub fn run_refresh(purpose: &str, unit: Option<&str>) -> Result<PmOperationStart> {
    let mut args = args(&["run", "refresh", "--json", "--purpose", purpose]);
    push_unit(&mut args, unit);
    run_operation("refresh", &args)
}

fn run_operation(operation: &str, args: &[String]) -> Result<PmOperationStart> {
    let raw: Value = json_payload(args, &["omactl.operation.started"])?;
    let returned_operation = raw
        .get("operation")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("omactl operation response missing operation"))?;
    if returned_operation != operation {
        bail!(
            "omactl operation response returned operation={returned_operation}, expected {operation}"
        );
    }
    let payload: OmactlUnitPayload =
        serde_json::from_value(raw.clone()).context("invalid omactl unit payload")?;
    Ok(PmOperationStart {
        unit: payload.unit,
        raw,
    })
}

pub fn status(unit: &str) -> Result<Value> {
    json_value(
        &args(&["status", "--json", unit]),
        &["omactl.unit.status", "omactl.status"],
    )
}

pub fn logs(unit: &str) -> Result<Value> {
    json_value(
        &args(&["logs", "--json", unit]),
        &["omactl.unit.logs", "omactl.logs"],
    )
}

pub fn result(unit: &str) -> Result<Value> {
    json_value(
        &args(&["result", "--json", unit]),
        &["omactl.unit.result", "omactl.result"],
    )
}

pub fn cancel(unit: &str) -> Result<Value> {
    json_value(
        &args(&["cancel", "--json", unit]),
        &["omactl.unit.cancel", "omactl.cancel"],
    )
}

pub fn validate_log_event(line: &str, unit: &str) -> Result<Value> {
    let value: Value = serde_json::from_str(line).context("invalid omactl log event JSON")?;
    let envelope: OmactlEnvelope =
        serde_json::from_value(value.clone()).context("invalid omactl log event envelope")?;
    if envelope.schema_version != SCHEMA_VERSION {
        bail!(
            "omactl log event schema_version={} expected {}",
            envelope.schema_version,
            SCHEMA_VERSION
        );
    }
    if !envelope.ok {
        return Err(omactl_error(envelope.kind, envelope.error).into());
    }
    if envelope.kind != "omactl.unit.log-line" {
        bail!("unexpected omactl log event kind: {}", envelope.kind);
    }
    let event_unit = envelope
        .data
        .as_ref()
        .and_then(|data| data.get("unit"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("omactl log event missing data.unit"))?;
    if event_unit != unit {
        bail!("omactl log event unit mismatch: {event_unit} != {unit}");
    }
    Ok(value)
}

/// Compatibility wrapper for the old command surface. Prefer pm_start_* commands.
pub fn run_oma(args: &[&str], wait: bool, follow: bool, unit: Option<&str>) -> Result<String> {
    if wait || follow {
        bail!("wait/follow are not supported by the omactl JSON compatibility wrapper");
    }
    match args.split_first() {
        Some((&"upgrade", rest)) => {
            let packages = package_args(rest);
            Ok(run_upgrade(&packages, contains_yes(rest), unit)?.unit)
        }
        Some((&"install", rest)) => {
            let packages = package_args(rest);
            Ok(run_install(&packages, contains_yes(rest), unit)?.unit)
        }
        Some((&"remove", rest)) => {
            let packages = package_args(rest);
            Ok(run_remove(
                &packages,
                contains_remove_config(rest),
                contains_yes(rest),
                unit,
            )?
            .unit)
        }
        Some((action, _)) => Err(anyhow!("unsupported omactl compatibility action: {action}")),
        None => Err(anyhow!("missing omactl compatibility action")),
    }
}

fn contains_yes(args: &[&str]) -> bool {
    args.iter().any(|arg| matches!(*arg, "--yes" | "-y"))
}

fn contains_remove_config(args: &[&str]) -> bool {
    args.iter()
        .any(|arg| matches!(*arg, "--remove-config" | "--remove_config" | "--purge"))
}

fn package_args(args: &[&str]) -> Vec<String> {
    args.iter()
        .filter(|arg| !arg.starts_with('-'))
        .map(|arg| (*arg).to_string())
        .collect()
}

fn count_packages(raw: &Value) -> usize {
    for key in ["packages", "upgradable", "updates", "items"] {
        if let Some(count) = raw.get(key).and_then(Value::as_array).map(Vec::len) {
            return count;
        }
    }
    raw.get("count")
        .or_else(|| raw.get("total"))
        .and_then(Value::as_u64)
        .map(|count| count as usize)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_success_envelope_by_kind() {
        let json = r#"{"schema_version":1,"ok":true,"kind":"omactl.query.upgradable","data":{"packages":[{"name":"apt"}]}}"#;
        let value: Value = parse_json(json, &["omactl.query.upgradable"]).unwrap();
        assert_eq!(count_packages(&value), 1);
    }

    #[test]
    fn preserves_structured_error_code() {
        let json = r#"{"schema_version":1,"ok":false,"kind":"omactl.query.upgradable","error":{"code":"UNSUPPORTED_COMMAND","message":"old omactl","details":{"capability":"query.upgradable.v1"}}}"#;
        let error = parse_json::<Value>(json, &["omactl.query.upgradable"])
            .unwrap_err()
            .to_string();
        assert!(error.contains("UNSUPPORTED_COMMAND"));
        assert!(error.contains("query.upgradable.v1"));
    }

    #[test]
    fn rejects_unexpected_kind() {
        let json = r#"{"schema_version":1,"ok":true,"kind":"omactl.query.installed","data":{}}"#;
        let error = parse_json::<Value>(json, &["omactl.query.upgradable"])
            .unwrap_err()
            .to_string();
        assert!(error.contains("omactl.query.installed"));
    }

    #[test]
    fn rejects_versioned_capabilities_kind() {
        let json = r#"{"schema_version":1,"ok":true,"kind":"omactl.capabilities.v1","data":{"capabilities":[]}}"#;
        let error = parse_exact_json::<Value>(json, "omactl.capabilities")
            .unwrap_err()
            .to_string();
        assert!(error.contains("omactl.capabilities.v1"));
    }

    #[test]
    fn rejects_error_log_event_without_payload() {
        let json = r#"{"schema_version":1,"ok":false,"kind":"omactl.unit.log-error"}"#;
        let error = validate_log_event(json, "oma-task-test.service")
            .unwrap_err()
            .to_string();
        assert!(error.contains("did not include structured error payload"));
    }

    #[test]
    fn extracts_empty_tum_updates() {
        let plan = json!({"tum":{"available":true,"updates":[]}});
        let updates = extract_tum_updates(&plan).unwrap();
        assert!(updates.is_empty());
    }

    #[test]
    fn rejects_unavailable_tum_updates() {
        let plan = json!({"tum":{"available":false,"updates":[]}});
        let error = extract_tum_updates(&plan).unwrap_err().to_string();
        assert!(error.contains("TUM_UNAVAILABLE"));
    }

    #[test]
    fn rejects_tum_updates_without_security_flag() {
        let plan = json!({
            "tum": {
                "available": true,
                "updates": [{
                    "manifest_name": "security",
                    "name": {"en": "Security updates"},
                    "package_count": 1,
                    "package_names": ["openssl"]
                }]
            }
        });
        let error = extract_tum_updates(&plan).unwrap_err().to_string();
        assert!(error.contains("invalid TUM update group payload"));
    }
}

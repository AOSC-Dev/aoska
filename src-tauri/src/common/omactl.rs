use crate::common::omactl_types::{
    OmactlCapabilitiesPayload, OmactlEnvelope, OmactlErrorPayload, OmactlUnitPayload,
    PmCapabilities, PmOperationStart, PmUpdateSummary, SCHEMA_VERSION,
};
use crate::common::utils::run_cmd;
use anyhow::{anyhow, bail, Context, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::path::Path;
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

pub fn is_busy() -> bool {
    let lock_path = "/run/lock/oma.lock";
    Path::new(lock_path).exists()
}

fn run_omactl(args: &[String]) -> Result<String> {
    run_cmd({
        let mut c = Command::new("omactl");
        c.args(args);
        c
    })
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

    let accepted = accepted_kinds(expected_kinds);
    if !accepted.iter().any(|k| k == &envelope.kind) {
        return Err(OmactlJsonError::UnexpectedKind {
            actual: envelope.kind,
            expected: accepted,
        }
        .into());
    }

    if !envelope.ok {
        return Err(omactl_error(envelope.kind, envelope.error).into());
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

pub fn capabilities() -> Result<PmCapabilities> {
    let raw = json_value(&args(&["capabilities", "--json"]), &["omactl.capabilities"])?;
    let payload: OmactlCapabilitiesPayload =
        serde_json::from_value(raw.clone()).context("invalid omactl capabilities payload")?;
    Ok(PmCapabilities {
        capabilities: payload.capabilities,
        raw,
    })
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

pub fn run_upgrade(packages: &[String], assume_yes: bool) -> Result<PmOperationStart> {
    run_operation(
        &run_args("upgrade", packages, assume_yes),
        &["omactl.run.upgrade"],
    )
}

pub fn run_install(packages: &[String], assume_yes: bool) -> Result<PmOperationStart> {
    if packages.is_empty() {
        bail!("packages is empty");
    }
    run_operation(
        &run_args("install", packages, assume_yes),
        &["omactl.run.install"],
    )
}

pub fn run_remove(
    packages: &[String],
    remove_config: bool,
    assume_yes: bool,
) -> Result<PmOperationStart> {
    if packages.is_empty() {
        bail!("packages is empty");
    }
    let mut args = run_args("remove", packages, assume_yes);
    if remove_config {
        args.insert(3, "--remove-config".to_string());
    }
    run_operation(&args, &["omactl.run.remove"])
}

pub fn run_refresh() -> Result<PmOperationStart> {
    run_operation(
        &args(&["run", "refresh", "--json"]),
        &["omactl.run.refresh"],
    )
}

fn run_operation(args: &[String], expected_kinds: &[&str]) -> Result<PmOperationStart> {
    let raw: Value = json_payload(args, expected_kinds)?;
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

/// Compatibility wrapper for the old command surface. Prefer pm_start_* commands.
pub fn run_oma(args: &[&str], _wait: bool, _follow: bool, _unit: Option<&str>) -> Result<String> {
    match args.split_first() {
        Some((&"upgrade", rest)) => {
            let packages = package_args(rest);
            Ok(run_upgrade(&packages, contains_yes(rest))?.unit)
        }
        Some((&"install", rest)) => {
            let packages = package_args(rest);
            Ok(run_install(&packages, contains_yes(rest))?.unit)
        }
        Some((&"remove", rest)) => {
            let packages = package_args(rest);
            Ok(run_remove(&packages, contains_remove_config(rest), contains_yes(rest))?.unit)
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
}

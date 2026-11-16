use super::amo_types::*;
use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use futures_util::StreamExt;
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use tokio::sync::{Mutex as AsyncMutex, OnceCell};
use zbus::{Connection, Proxy};

const SERVICE: &str = "io.aosc.Amo";
const PATH: &str = "/io/aosc/Amo";
const INTERFACE: &str = "io.aosc.Amo1";
const MAX_LOG_LINES: usize = 300;

#[derive(Default)]
pub struct AmoBackend {
    connection: OnceCell<Connection>,
    // amo rejects concurrent planning requests with its package-operation lock.
    queries: AsyncMutex<()>,
    operations: Arc<Mutex<HashMap<String, Operation>>>,
}

impl AmoBackend {
    async fn connection(&self) -> Result<Connection> {
        Ok(self
            .connection
            .get_or_try_init(|| async {
                Connection::system()
                    .await
                    .context("AMO_UNAVAILABLE: cannot connect to the system bus")
            })
            .await?
            .clone())
    }

    async fn proxy(connection: &Connection) -> Result<Proxy<'static>> {
        Proxy::new(connection, SERVICE, PATH, INTERFACE)
            .await
            .context("AMO_UNAVAILABLE: cannot connect to io.aosc.Amo1")
    }

    pub async fn capabilities(&self) -> Result<PmCapabilities> {
        let connection = self.connection().await?;
        let introspection = Proxy::new(
            &connection,
            SERVICE,
            PATH,
            "org.freedesktop.DBus.Introspectable",
        )
        .await?;
        let xml: String = introspection
            .call("Introspect", &())
            .await
            .context("AMO_UNAVAILABLE: amo is not running on the system bus")?;
        for member in [
            INTERFACE,
            "Search",
            "UpdatesList",
            "GetTransaction",
            "ApplyChanges",
            "Refresh",
            "Status",
            "ResultReport",
        ] {
            if !xml.contains(&format!("name=\"{member}\"")) {
                bail!("UNSUPPORTED_BACKEND: amo is missing {member}");
            }
        }
        Ok(PmCapabilities {
            capabilities: [
                "amo.backend.v1",
                "query.upgradable.v1",
                "query.package-detail.v1",
                "plan.install.v1",
                "plan.remove.v1",
                "plan.upgrade.v1",
                "plan.upgrade.selected.v1",
                "run.install.v1",
                "run.remove.v1",
                "run.upgrade.v1",
                "run.upgrade.selected.v1",
                "run.refresh.v1",
                "plan.tum.v1",
                "operation.status.v1",
                "operation.result.v1",
                "operation.logs.v1",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            raw: json!({"backend":"amo", "service":SERVICE, "path":PATH, "interface":INTERFACE}),
        })
    }

    async fn updates_transaction(&self) -> Result<Transaction> {
        self.read_transaction(None).await
    }

    async fn read_transaction(&self, request: Option<&TransactionRequest>) -> Result<Transaction> {
        let _guard = self.queries.lock().await;
        let connection = self.connection().await?;
        let proxy = Self::proxy(&connection).await?;
        let mut retries = 0;
        loop {
            let response: zbus::Result<String> = match request {
                Some(request) => {
                    proxy
                        .call(
                            "GetTransaction",
                            &(&request.install, &request.remove, request.upgrade),
                        )
                        .await
                }
                None => proxy.call("UpdatesList", &()).await,
            };
            match response {
                Err(zbus::Error::MethodError(ref name, ref message, _))
                    if name.as_str() == "org.freedesktop.DBus.Error.Failed"
                        && message.as_deref() == Some("Another task is already running!")
                        && retries < 10 =>
                {
                    // Plasma also queries amo immediately after UpdatesChanged.
                    retries += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                response => {
                    let raw = response.context("amo transaction query failed")?;
                    return serde_json::from_str(&raw)
                        .context("MALFORMED_AMO_RESPONSE: invalid transaction");
                }
            }
        }
    }

    pub async fn list_updates(&self) -> Result<PmPackageList> {
        Ok(PmPackageList {
            packages: updates_from_transaction(&self.updates_transaction().await?),
        })
    }

    pub async fn update_summary(&self) -> Result<PmUpdateSummary> {
        Ok(summary_from_transaction(&self.updates_transaction().await?))
    }

    pub async fn tum_updates(&self) -> Result<Vec<TumUpdateInfo>> {
        let transaction = self.updates_transaction().await?;
        let updates = updates_from_transaction(&transaction);
        Ok(transaction
            .tum
            .iter()
            .map(|topic| {
                let names = topic_packages(topic, &transaction.tum);
                let package_names: Vec<_> = updates
                    .iter()
                    .filter(|pkg| names.contains(pkg.name.split(':').next().unwrap_or(&pkg.name)))
                    .map(|pkg| pkg.name.clone())
                    .collect();
                TumUpdateInfo {
                    manifest_name: topic.id.clone(),
                    name: topic.name.clone(),
                    is_security: topic.security,
                    package_count: package_names.len(),
                    package_names,
                    caution: topic.caution.clone(),
                }
            })
            .collect())
    }

    pub async fn package_state(&self, packages: &[String]) -> Result<PmPackageList> {
        if packages.is_empty() {
            bail!("packages is empty");
        }
        let _guard = self.queries.lock().await;
        let connection = self.connection().await?;
        let proxy = Self::proxy(&connection).await?;
        let mut states = Vec::new();
        for name in packages {
            let raw: zbus::Result<String> = proxy.call("Search", &(name,)).await;
            let results: Vec<SearchResult> = match raw {
                Ok(raw) => serde_json::from_str(&raw)
                    .context("MALFORMED_AMO_RESPONSE: invalid search results")?,
                Err(zbus::Error::MethodError(error, message, _))
                    if error.as_str() == "org.freedesktop.DBus.Error.Failed"
                        && message
                            .as_deref()
                            .is_some_and(|text| text.starts_with("No result found:")) =>
                {
                    Vec::new()
                }
                Err(error) => return Err(error).context("amo Search failed"),
            };
            states.push(package_from_search(name, &results));
        }
        Ok(PmPackageList { packages: states })
    }

    pub async fn plan(&self, operation: &str, packages: &[String]) -> Result<PmPlan> {
        let request = transaction_request(operation, packages)?;
        let transaction = self.read_transaction(Some(&request)).await?;
        Ok(plan_from_transaction(operation, packages, &transaction))
    }

    pub async fn start(&self, operation: &str, packages: &[String]) -> Result<PmOperationStart> {
        let request = if operation == "refresh" {
            None
        } else {
            Some(transaction_request(operation, packages)?)
        };
        let _guard = self.queries.lock().await;
        if self
            .operations
            .lock()
            .unwrap()
            .values()
            .any(|op| matches!(op.status.state.as_str(), "running" | "unknown"))
        {
            bail!("OMA_BUSY: an aoska package operation is still running or its result is unknown");
        }
        let connection = self.connection().await?;
        let proxy = Self::proxy(&connection).await?;
        // Subscribe before dispatch: a no-op can report its result before the method reply.
        let mut statuses = proxy.receive_signal("Status").await?;
        let mut reports = proxy.receive_signal("ResultReport").await?;
        let mut owners = proxy.receive_owner_changed().await?;
        let helper = if request.is_some() {
            Some(start_debconf_helper()?)
        } else {
            None
        };
        let response = match request {
            Some(request) => {
                proxy
                    .call_method(
                        "ApplyChanges",
                        &(&request.install, &request.remove, request.upgrade),
                    )
                    .await
            }
            None => proxy.call_method("Refresh", &()).await,
        };
        let response = match response {
            Ok(response) => response,
            Err(error) => {
                stop_debconf_helper(helper);
                return Err(error).context("amo rejected the operation");
            }
        };
        let id: u64 = response.body().deserialize()?;
        let owner = response.header().sender().map(|name| name.to_string());
        let record = Operation::new(id, operation, packages.to_vec());
        let start = record.start.clone();
        self.operations
            .lock()
            .unwrap()
            .insert(start.id.clone(), record);
        let operations = self.operations.clone();
        let key = start.id.clone();
        tokio::spawn(async move {
            let mut received_result = false;
            let observation = async {
                loop {
                    tokio::select! {
                        biased;
                        report = reports.next() => {
                            let message = report.context("amo ResultReport stream closed")?;
                            let raw: String = message.body().deserialize()?;
                            let report: ResultReport = serde_json::from_str(&raw).context("invalid amo ResultReport")?;
                            if operations.lock().unwrap().get_mut(&key).unwrap().report(report) {
                                received_result = true;
                                return Ok::<(), anyhow::Error>(());
                            }
                        }
                        changed = owners.next() => {
                            let current = changed.flatten().map(|name| name.to_string());
                            if current != owner {
                                bail!("amo service disconnected or restarted; its operation result is unavailable");
                            }
                        }
                        status = statuses.next() => {
                            let message = status.context("amo Status stream closed")?;
                            let raw: String = message.body().deserialize()?;
                            operations.lock().unwrap().get_mut(&key).unwrap().progress(&raw);
                        }
                    }
                }
            }.await;
            if let Err(error) = observation {
                let mut state = operations.lock().unwrap();
                let record = state.get_mut(&key).unwrap();
                record.status.state = "unknown".into();
                record.status.message = Some(format!(
                    "Cannot observe amo result; the operation may still be running: {error}"
                ));
            }
            if received_result {
                stop_debconf_helper(helper);
            }
            // If observation is lost, leave the configuration dialog alive for any in-flight dpkg work.
        });
        Ok(start)
    }

    pub fn status(&self, id: &str) -> Result<PmOperationStatus> {
        self.operations
            .lock()
            .unwrap()
            .get(id)
            .map(|op| op.status.clone())
            .ok_or_else(|| anyhow!("UNKNOWN_OPERATION: {id}; amo has no operation history API"))
    }

    pub fn logs(&self, id: &str) -> Result<PmOperationLogs> {
        self.operations
            .lock()
            .unwrap()
            .get(id)
            .map(|op| PmOperationLogs {
                id: id.into(),
                lines: op.logs.iter().cloned().collect(),
            })
            .ok_or_else(|| anyhow!("UNKNOWN_OPERATION: {id}"))
    }
}

fn start_debconf_helper() -> Result<Child> {
    if std::env::var_os("DISPLAY").is_none() && std::env::var_os("WAYLAND_DISPLAY").is_none() {
        bail!("DEBCONF_UNAVAILABLE: start aoska in a graphical session for package configuration dialogs");
    }
    Command::new("debconf-kde-helper")
        .args(["--socket-path", "/tmp/amo-debconf-sock"])
        .stdin(Stdio::null())
        .spawn()
        .context("DEBCONF_UNAVAILABLE: install debconf-kde before changing packages")
}

fn stop_debconf_helper(helper: Option<Child>) {
    if let Some(mut child) = helper {
        let _ = child.kill();
        let _ = child.wait();
    }
}

struct TransactionRequest {
    install: Vec<String>,
    remove: Vec<String>,
    upgrade: bool,
}

fn transaction_request(operation: &str, packages: &[String]) -> Result<TransactionRequest> {
    match operation {
        "install" | "remove" if packages.is_empty() => bail!("packages is empty"),
        "install" => Ok(TransactionRequest {
            install: packages.to_vec(),
            remove: vec![],
            upgrade: false,
        }),
        "remove" => Ok(TransactionRequest {
            install: vec![],
            remove: packages.to_vec(),
            upgrade: false,
        }),
        "update" => Ok(TransactionRequest {
            install: packages.to_vec(),
            remove: vec![],
            upgrade: packages.is_empty(),
        }),
        _ => bail!("Unsupported package operation: {operation}"),
    }
}

fn package_from_search(name: &str, results: &[SearchResult]) -> PmPackage {
    let found = results.iter().find(|pkg| pkg.name == name);
    PmPackage {
        name: name.into(),
        display_name: name.into(),
        package_name: name.into(),
        installed: found.is_some_and(|pkg| pkg.status != SearchStatus::Avail),
        upgradable: found.is_some_and(|pkg| pkg.status == SearchStatus::Upgrade),
        openable: false,
        aoska_manageable: found.is_some(),
        requires_manual_action: found.is_none(),
        current_version: found.and_then(|pkg| pkg.old_version.clone()),
        new_version: found.map(|pkg| pkg.new_version.clone()),
        installed_size: None,
        download_size: None,
        architecture: None,
        is_security: false,
    }
}

fn topic_packages(topic: &TumEntry, topics: &[TumEntry]) -> BTreeSet<String> {
    let mut packages: BTreeSet<_> = topic.packages.iter().cloned().collect();
    for id in &topic.topics {
        if let Some(child) = topics.iter().find(|entry| &entry.id == id) {
            packages.extend(child.packages.iter().cloned());
        }
    }
    packages
}

fn updates_from_transaction(transaction: &Transaction) -> Vec<PmPackage> {
    let security: BTreeSet<_> = transaction
        .tum
        .iter()
        .filter(|topic| topic.security)
        .flat_map(|topic| topic_packages(topic, &transaction.tum))
        .collect();
    transaction
        .install
        .iter()
        .filter(|pkg| {
            pkg.old_version.is_some() && matches!(pkg.op.as_str(), "Upgrade" | "Downgrade")
        })
        .map(|pkg| PmPackage {
            name: pkg.name.clone(),
            display_name: pkg.name_without_arch.clone(),
            package_name: pkg.name.clone(),
            installed: true,
            upgradable: true,
            openable: false,
            aoska_manageable: true,
            requires_manual_action: false,
            current_version: pkg.old_version.clone(),
            new_version: Some(pkg.new_version.clone()),
            installed_size: pkg.old_size,
            download_size: Some(pkg.download_size),
            architecture: Some(pkg.arch.clone()),
            is_security: security.contains(&pkg.name_without_arch),
        })
        .collect()
}

fn summary_from_transaction(transaction: &Transaction) -> PmUpdateSummary {
    let packages = updates_from_transaction(transaction);
    let available = !transaction.tum.is_empty();
    PmUpdateSummary {
        total: packages.len(),
        security: available.then(|| packages.iter().filter(|pkg| pkg.is_security).count()),
        security_classification_available: available,
        system: 0,
        software: packages.len(),
        stale: false,
    }
}

fn plan_from_transaction(
    operation: &str,
    packages: &[String],
    transaction: &Transaction,
) -> PmPlan {
    PmPlan {
        operation: operation.into(),
        packages: packages.to_vec(),
        install: transaction
            .install
            .iter()
            .map(|pkg| PmPlanEntry {
                name: pkg.name.clone(),
                current_version: pkg.old_version.clone(),
                new_version: Some(pkg.new_version.clone()),
                download_size: Some(pkg.download_size),
                installed_size: Some(pkg.new_size),
            })
            .collect(),
        remove: transaction
            .remove
            .iter()
            .map(|pkg| PmPlanEntry {
                name: pkg.name.clone(),
                current_version: pkg.version.clone(),
                new_version: None,
                download_size: None,
                installed_size: Some(pkg.size),
            })
            .collect(),
        total_download_size: transaction.total_download_size,
        disk_size_delta: transaction.disk_size_delta,
    }
}

struct Operation {
    request_id: u64,
    start: PmOperationStart,
    status: PmOperationStatus,
    logs: VecDeque<String>,
}

impl Operation {
    fn new(request_id: u64, operation: &str, packages: Vec<String>) -> Self {
        let id = request_id.to_string();
        let started_at = Utc::now().to_rfc3339();
        Self {
            request_id,
            start: PmOperationStart {
                id: id.clone(),
                operation: operation.into(),
                packages,
                started_at: started_at.clone(),
            },
            status: PmOperationStatus {
                id,
                operation: operation.into(),
                state: "running".into(),
                started_at,
                finished_at: None,
                message: None,
            },
            logs: VecDeque::from([format!("amo accepted {operation} request {request_id}")]),
        }
    }

    fn progress(&mut self, raw: &str) {
        if let Ok(value) = serde_json::from_str::<Value>(raw) {
            if value
                .get("request_id")
                .and_then(Value::as_u64)
                .is_some_and(|id| id != self.request_id)
            {
                return;
            }
        }
        // Most amo Status events are global diagnostics, without a request ID.
        if self.logs.len() == MAX_LOG_LINES {
            self.logs.pop_front();
        }
        self.logs.push_back(format!("amo activity: {raw}"));
    }

    fn report(&mut self, report: ResultReport) -> bool {
        if report.request_id != self.request_id {
            return false;
        }
        let (state, message) = match report.status {
            TaskResult::Success => (
                "succeeded",
                "amo completed the operation and refreshed package state".into(),
            ),
            TaskResult::Failed(reason) => ("failed", reason),
        };
        self.status.state = state.into();
        self.status.message = Some(message.clone());
        self.status.finished_at = Some(Utc::now().to_rfc3339());
        if self.logs.len() == MAX_LOG_LINES {
            self.logs.pop_front();
        }
        self.logs.push_back(message);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transaction_json() -> Value {
        serde_json::json!({
            "install": [
                {"name":"firefox:amd64","name_without_arch":"firefox","old_version":"159","new_version":"160","new_size":400,"download_size":100,"arch":"amd64","op":"Upgrade"},
                {"name":"lib-example","name_without_arch":"lib-example","old_version":null,"new_version":"1","new_size":40,"download_size":10,"arch":"amd64","op":"Install"}
            ],
            "remove": [], "disk_size_delta": 20, "total_download_size": 110,
            "tum": [
                {"id":"browser","security":true,"name":{"default":"Browser security"},"packages":["firefox"],"topics":[],"caution":null},
                {"id":"all-security","security":true,"name":{"default":"Security"},"packages":[],"topics":["browser"],"caution":null}
            ]
        })
    }

    fn transaction() -> Transaction {
        serde_json::from_value(transaction_json()).unwrap()
    }

    #[test]
    fn maps_installed_and_available_search_results_without_guessing_from_candidate() {
        let results: Vec<SearchResult> = serde_json::from_str(r#"[
            {"name":"firefox-esr","desc":"Browser","old_version":null,"new_version":"153","status":"Avail","full_match":true},
            {"name":"firefox","desc":"Browser","old_version":"159","new_version":"160","status":"Upgrade","full_match":true}
        ]"#).unwrap();
        let pkg = package_from_search("firefox-esr", &results);
        assert!(!pkg.installed);
        assert!(!pkg.upgradable);
        assert!(!pkg.openable);
        assert!(pkg.aoska_manageable);
        let pkg = package_from_search("firefox", &results);
        assert!(pkg.installed && pkg.upgradable);
        assert_eq!(pkg.current_version.as_deref(), Some("159"));
        let absent = package_from_search("virtual-browser", &results);
        assert!(!absent.aoska_manageable);
        assert!(absent.requires_manual_action);
    }

    #[test]
    fn counts_only_updates_and_deduplicates_security_topics() {
        let tx = transaction();
        let packages = updates_from_transaction(&tx);
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].name, "firefox:amd64");
        assert!(packages[0].is_security);
        let summary = summary_from_transaction(&tx);
        assert_eq!(summary.total, 1);
        assert_eq!(summary.security, Some(1));
        assert_eq!(summary.software, 1);
    }

    #[test]
    fn empty_tum_does_not_claim_security_metadata_is_available() {
        let mut tx = transaction();
        tx.tum.clear();
        let summary = summary_from_transaction(&tx);
        assert_eq!(summary.security, None);
        assert!(!summary.security_classification_available);
    }

    #[test]
    fn selected_updates_use_install_candidates_and_only_full_updates_set_upgrade() {
        let names = vec!["firefox".to_string()];
        let selected = transaction_request("update", &names).unwrap();
        assert_eq!(selected.install, names);
        assert!(selected.remove.is_empty());
        assert!(!selected.upgrade);
        assert!(transaction_request("update", &[]).unwrap().upgrade);
        assert_eq!(transaction_request("remove", &names).unwrap().remove, names);
        assert!(transaction_request("install", &[]).is_err());
        assert!(transaction_request("remove", &[]).is_err());
    }

    #[test]
    fn preview_keeps_dependency_changes_and_sizes() {
        let plan = plan_from_transaction("update", &["firefox".into()], &transaction());
        assert_eq!(plan.install.len(), 2);
        assert_eq!(plan.total_download_size, 110);
        assert_eq!(plan.install[1].name, "lib-example");
    }

    #[test]
    fn request_ids_remain_exact_strings_and_only_matching_results_finish() {
        let id = 87020904000000001_u64;
        let mut operation = Operation::new(id, "install", vec!["firefox".into()]);
        operation.progress(r#"{"status":"finished","request_id":87020904000000001}"#);
        assert_eq!(operation.status.state, "running");
        assert!(!operation.report(
            serde_json::from_str(r#"{"request_id":87020904000000002,"status":"Success"}"#).unwrap()
        ));
        assert!(operation.report(
            serde_json::from_str(r#"{"request_id":87020904000000001,"status":"Success"}"#).unwrap()
        ));
        assert_eq!(operation.status.state, "succeeded");
        assert_eq!(
            serde_json::to_value(&operation.start).unwrap()["id"],
            "87020904000000001"
        );
    }

    #[test]
    fn operation_failure_preserves_amo_reason() {
        let mut operation = Operation::new(42, "remove", vec!["hello".into()]);
        operation.report(ResultReport {
            request_id: 42,
            status: TaskResult::Failed("dpkg failed".into()),
        });
        assert_eq!(operation.status.state, "failed");
        assert_eq!(operation.status.message.as_deref(), Some("dpkg failed"));
    }

    // This test owns a private bus and never invokes the system package manager.
    #[cfg(target_os = "linux")]
    mod dbus_contract {
        use super::*;
        use std::io::{BufRead, BufReader};
        use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
        use std::time::Duration;
        use zbus::object_server::SignalEmitter;

        struct TestBus {
            child: Child,
            address: String,
        }

        impl TestBus {
            fn new() -> Self {
                let mut child = Command::new("dbus-daemon")
                    .args(["--session", "--nofork", "--print-address=1"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("dbus-daemon is required for Linux D-Bus tests");
                let mut address = String::new();
                BufReader::new(child.stdout.take().unwrap())
                    .read_line(&mut address)
                    .unwrap();
                Self {
                    child,
                    address: address.trim().into(),
                }
            }
        }

        impl Drop for TestBus {
            fn drop(&mut self) {
                let _ = self.child.kill();
                let _ = self.child.wait();
            }
        }

        type PlannedArguments = (Vec<String>, Vec<String>, bool);

        struct FakeAmo {
            next_id: AtomicU64,
            mode: Arc<AtomicU8>,
            busy_reads: Arc<Mutex<u8>>,
            plans: Arc<Mutex<Vec<PlannedArguments>>>,
        }

        #[zbus::interface(name = "io.aosc.Amo1")]
        impl FakeAmo {
            fn updates_list(&self) -> zbus::fdo::Result<String> {
                let mut remaining = self.busy_reads.lock().unwrap();
                if *remaining > 0 {
                    *remaining -= 1;
                    return Err(zbus::fdo::Error::Failed(
                        "Another task is already running!".into(),
                    ));
                }
                Ok(transaction_json().to_string())
            }

            fn search(&self, query: &str) -> zbus::fdo::Result<String> {
                if query == "absent" {
                    return Err(zbus::fdo::Error::Failed("No result found: absent".into()));
                }
                if query == "denied" {
                    return Err(zbus::fdo::Error::AccessDenied("Query denied".into()));
                }
                Ok(r#"[{"name":"firefox","desc":"Browser","old_version":"159","new_version":"160","status":"Upgrade"}]"#.into())
            }

            fn get_transaction(
                &self,
                install: Vec<String>,
                remove: Vec<String>,
                upgrade: bool,
            ) -> String {
                self.plans.lock().unwrap().push((install, remove, upgrade));
                transaction_json().to_string()
            }

            fn apply_changes(
                &self,
                _install: Vec<String>,
                _remove: Vec<String>,
                _upgrade: bool,
            ) -> zbus::fdo::Result<u64> {
                Err(zbus::fdo::Error::AccessDenied(
                    "Test daemon rejects mutations".into(),
                ))
            }

            async fn refresh(
                &self,
                #[zbus(signal_context)] emitter: SignalEmitter<'_>,
            ) -> zbus::fdo::Result<u64> {
                let id = self.next_id.fetch_add(1, Ordering::SeqCst);
                if self.mode.load(Ordering::SeqCst) != 2 {
                    Self::status(
                        &emitter,
                        json!({"status":"finished","request_id":id}).to_string(),
                    )
                    .await?;
                    // Exercise result buffering before the D-Bus method returns, including a different request's result.
                    Self::result_report(
                        &emitter,
                        json!({"request_id":id + 100,"status":"Success"}).to_string(),
                    )
                    .await?;
                    let status = if self.mode.load(Ordering::SeqCst) == 1 {
                        json!({"Failed":"fixture refresh failure"})
                    } else {
                        json!("Success")
                    };
                    Self::result_report(
                        &emitter,
                        json!({"request_id":id,"status":status}).to_string(),
                    )
                    .await?;
                }
                Ok(id)
            }

            #[zbus(signal)]
            async fn status(emitter: &SignalEmitter<'_>, value: String) -> zbus::Result<()>;
            #[zbus(signal)]
            async fn result_report(emitter: &SignalEmitter<'_>, value: String) -> zbus::Result<()>;
        }

        async fn wait_state(backend: &AmoBackend, id: &str, expected: &str) -> PmOperationStatus {
            tokio::time::timeout(Duration::from_secs(5), async {
                loop {
                    let status = backend.status(id).unwrap();
                    if status.state == expected {
                        break status;
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .expect("operation did not reach the expected state")
        }

        #[tokio::test]
        async fn private_bus_queries_plans_early_results_and_service_loss() {
            let bus = TestBus::new();
            let mode = Arc::new(AtomicU8::new(0));
            let busy_reads = Arc::new(Mutex::new(2));
            let plans = Arc::new(Mutex::new(Vec::new()));
            let service = zbus::connection::Builder::address(bus.address.as_str())
                .unwrap()
                .name(SERVICE)
                .unwrap()
                .serve_at(
                    PATH,
                    FakeAmo {
                        next_id: AtomicU64::new(87020904000000001),
                        mode: mode.clone(),
                        busy_reads: busy_reads.clone(),
                        plans: plans.clone(),
                    },
                )
                .unwrap()
                .build()
                .await
                .unwrap();
            let connection = zbus::connection::Builder::address(bus.address.as_str())
                .unwrap()
                .build()
                .await
                .unwrap();
            let backend = AmoBackend::default();
            backend.connection.set(connection).unwrap();
            let capabilities = backend.capabilities().await.unwrap();
            assert!(capabilities
                .capabilities
                .iter()
                .any(|cap| cap == "amo.backend.v1"));
            assert!(!capabilities
                .capabilities
                .iter()
                .any(|cap| cap.contains("cancel") || cap == "query.installed.v1"));
            let (summary, updates) = tokio::join!(backend.update_summary(), backend.list_updates());
            assert_eq!(summary.unwrap().total, updates.unwrap().packages.len());
            assert_eq!(*busy_reads.lock().unwrap(), 0);
            assert_eq!(
                backend.tum_updates().await.unwrap()[0].package_names,
                vec!["firefox:amd64"]
            );
            assert!(
                backend
                    .package_state(&["firefox".into()])
                    .await
                    .unwrap()
                    .packages[0]
                    .installed
            );
            assert!(
                !backend
                    .package_state(&["absent".into()])
                    .await
                    .unwrap()
                    .packages[0]
                    .aoska_manageable
            );
            assert!(backend
                .package_state(&["denied".into()])
                .await
                .unwrap_err()
                .to_string()
                .contains("Search"));
            backend.plan("update", &["firefox".into()]).await.unwrap();
            backend.plan("update", &[]).await.unwrap();
            let captured = plans.lock().unwrap().clone();
            assert_eq!(captured[0], (vec!["firefox".into()], vec![], false));
            assert_eq!(captured[1], (vec![], vec![], true));

            let first = backend.start("refresh", &[]).await.unwrap();
            assert_eq!(first.id, "87020904000000001");
            wait_state(&backend, &first.id, "succeeded").await;
            mode.store(1, Ordering::SeqCst);
            let failed = backend.start("refresh", &[]).await.unwrap();
            assert_eq!(
                wait_state(&backend, &failed.id, "failed")
                    .await
                    .message
                    .as_deref(),
                Some("fixture refresh failure")
            );
            mode.store(2, Ordering::SeqCst);
            let interrupted = backend.start("refresh", &[]).await.unwrap();
            assert!(backend
                .start("refresh", &[])
                .await
                .unwrap_err()
                .to_string()
                .contains("OMA_BUSY"));
            service.release_name(SERVICE).await.unwrap();
            let unknown = wait_state(&backend, &interrupted.id, "unknown").await;
            assert!(unknown
                .message
                .unwrap()
                .contains("disconnected or restarted"));
            assert!(backend.status("not-a-request").is_err());
        }
    }
}

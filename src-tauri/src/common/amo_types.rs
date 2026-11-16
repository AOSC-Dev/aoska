use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub name: String,
    #[serde(default)]
    pub desc: String,
    pub old_version: Option<String>,
    pub new_version: String,
    pub status: SearchStatus,
    #[serde(default)]
    pub is_base: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum SearchStatus {
    Avail,
    Installed,
    Upgrade,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstallEntry {
    pub name: String,
    pub name_without_arch: String,
    pub old_version: Option<String>,
    pub new_version: String,
    pub old_size: Option<u64>,
    pub new_size: u64,
    pub download_size: u64,
    pub arch: String,
    pub op: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveEntry {
    pub name: String,
    pub version: Option<String>,
    pub size: u64,
    #[serde(default)]
    pub details: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub install: Vec<InstallEntry>,
    pub remove: Vec<RemoveEntry>,
    pub disk_size_delta: i64,
    pub total_download_size: u64,
    #[serde(default)]
    pub tum: Vec<TumEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TumEntry {
    pub id: String,
    pub security: bool,
    pub name: BTreeMap<String, String>,
    #[serde(default)]
    pub packages: Vec<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    pub caution: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultReport {
    pub request_id: u64,
    pub status: TaskResult,
}

#[derive(Debug, Clone, Deserialize)]
pub enum TaskResult {
    Success,
    Failed(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct PmCapabilities {
    pub capabilities: Vec<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmPackage {
    pub name: String,
    pub display_name: String,
    pub package_name: String,
    pub installed: bool,
    pub upgradable: bool,
    pub openable: bool,
    pub aoska_manageable: bool,
    pub requires_manual_action: bool,
    pub current_version: Option<String>,
    pub new_version: Option<String>,
    pub installed_size: Option<u64>,
    pub download_size: Option<u64>,
    pub architecture: Option<String>,
    pub is_security: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PmPackageList {
    pub packages: Vec<PmPackage>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmUpdateSummary {
    pub total: usize,
    pub security: Option<usize>,
    pub security_classification_available: bool,
    pub system: usize,
    pub software: usize,
    pub stale: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmPlanEntry {
    pub name: String,
    pub current_version: Option<String>,
    pub new_version: Option<String>,
    pub download_size: Option<u64>,
    pub installed_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmPlan {
    pub operation: String,
    pub packages: Vec<String>,
    pub install: Vec<PmPlanEntry>,
    pub remove: Vec<PmPlanEntry>,
    pub total_download_size: u64,
    pub disk_size_delta: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmOperationStart {
    pub id: String,
    pub operation: String,
    pub packages: Vec<String>,
    pub started_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmOperationStatus {
    pub id: String,
    pub operation: String,
    pub state: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PmOperationLogs {
    pub id: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TumUpdateInfo {
    pub manifest_name: String,
    pub name: BTreeMap<String, String>,
    pub is_security: bool,
    pub package_count: usize,
    pub package_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caution: Option<BTreeMap<String, String>>,
}

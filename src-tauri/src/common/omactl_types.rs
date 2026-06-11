use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

pub const SCHEMA_VERSION: u64 = 1;

#[derive(Debug, Clone, Deserialize)]
pub struct OmactlEnvelope {
    pub schema_version: u64,
    pub ok: bool,
    pub kind: String,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub error: Option<OmactlErrorPayload>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OmactlErrorPayload {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OmactlCapabilitiesPayload {
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OmactlUnitPayload {
    pub unit: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PmCapabilities {
    pub capabilities: Vec<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct PmOperationStart {
    pub unit: String,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct PmUpdateSummary {
    pub total: usize,
    pub security: Option<usize>,
    pub security_classification_available: bool,
    pub raw: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TumUpdateInfo {
    pub manifest_name: String,
    pub name: BTreeMap<String, String>,
    pub is_security: bool,
    pub package_count: u64,
    pub package_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<BTreeMap<String, String>>,
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

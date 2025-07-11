use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Working,
    Games,
    Video,
    Creating,
    Observing
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageBrief {
    pub name: String,
    pub intro: String,
    pub icon: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFlags {
    pub unoffical: bool,
    pub verified: bool,
    pub non_native: bool,
    pub windows_app: bool,
    pub telemetry: bool,
    pub service_limited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub publisher: String,
    pub source: String,
    pub version: String,
    pub inner_version: i32,
    pub update_date: String,
    pub install_size: i64,
    pub homepage: String,
}

/// The detailed infomation of a package. 
/// Will be serialized into repo-root/packages/$package_name/meta.json 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDetail {
    name: String,
    icon: PathBuf,
    title: String,
    intro: String,
    category: Category,
    screenshot: PathBuf,
    package_flags: PackageFlags,
    package_info: PackageInfo,
}

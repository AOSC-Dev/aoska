use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Category {
    Working,
    Games,
    Video,
    Creating,
    Observing,
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
    pub name: String,
    pub icon: PathBuf,
    pub title: String,
    pub intro: String,
    pub category: Category,
    pub screenshot: Vec<PathBuf>,
    pub package_flags: PackageFlags,
    pub package_info: PackageInfo,
}

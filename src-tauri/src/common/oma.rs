use ahash::HashMap;
use anyhow::{Ok, Result};
use oma_pm::{
    apt::{OmaApt, OmaAptError, OmaOperation},
    sort::SummarySort,
};
use oma_tum::{get_matches_tum, get_tum};

#[derive(Debug, Clone, serde::Serialize)]
pub struct TumUpdateInfo {
    pub manifest_name: String,
    pub name: HashMap<String, String>,
    pub is_security: bool,
    pub package_count: usize,
    pub package_names: Vec<String>,
    pub caution: Option<HashMap<String, String>>,
}

pub async fn check_upgradable(apt: &OmaApt) -> Result<OmaOperation, OmaAptError> {
    apt.upgrade(oma_pm::apt::Upgrade::FullUpgrade)?;
    apt.summary(
        SummarySort::default().operation().names(),
        |_| false,
        |_| false,
    )
}

pub async fn check_upgradable_count(apt: &OmaApt) -> Result<usize, OmaAptError> {
    apt.upgrade(oma_pm::apt::Upgrade::FullUpgrade)?;
    apt.count_pending_upgradable_pkgs()
}

pub async fn check_tum_upgradable(apt: &OmaApt) -> Result<Vec<TumUpdateInfo>> {
    let sysroot = std::path::Path::new("/");
    let tum_manifests = get_tum(sysroot)?;
    let operation = check_upgradable(apt).await?;

    let matched_manifests = get_matches_tum(&tum_manifests, &operation);

    let tum_update: Vec<TumUpdateInfo> = matched_manifests
        .into_iter()
        .map(|(manifest_name, entry_ref)| {
            let (package_names, caution, name) = match entry_ref {
                oma_tum::TopicUpdateEntryRef::Conventional {
                    packages,
                    caution,
                    name,
                    ..
                } => (
                    packages.keys().map(|name| name.to_string()).collect(),
                    caution,
                    name,
                ),
                // NOTE: Ignore Cumulative TUM since it hasn't been used yet.
                oma_tum::TopicUpdateEntryRef::Cumulative { caution, name, .. } => {
                    (Vec::new(), caution, name)
                }
            };
            TumUpdateInfo {
                manifest_name: manifest_name.to_string(),
                name: name.clone(),
                is_security: entry_ref.is_security(),
                package_count: entry_ref.count_packages(),
                package_names,
                caution: caution.cloned(),
            }
        })
        .collect();
    Ok(tum_update)
}

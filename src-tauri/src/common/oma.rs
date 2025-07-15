use anyhow::Result;
use oma_pm::{
    apt::{OmaApt, OmaAptArgs, OmaAptError, OmaOperation},
    sort::SummarySort,
};
use oma_tum::{get_matches_tum, get_tum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityUpdateInfo {
    pub manifest_name: String,
    pub is_security: bool,
    pub package_count: usize,
    pub package_names: Vec<String>,
}

pub async fn check_upgradable(apt: &OmaApt) -> Result<usize, OmaAptError> {
    apt.count_pending_upgradable_pkgs()
}

pub async fn check_upgradable_detail(apt: &OmaApt) -> Result<OmaOperation, OmaAptError> {
    apt.upgrade(oma_pm::apt::Upgrade::FullUpgrade)?;
    apt.summary(
        SummarySort::default().operation().names(),
        |_| false,
        |_| false,
    )
}

pub async fn check_security_upgradable(apt: &OmaApt) -> Result<usize> {
    let sysroot = std::path::Path::new("/");
    let tum_manifests = get_tum(sysroot)?;
    // dbg!(&tum_manifests);
    let operation = check_upgradable_detail(apt).await?;
    // dbg!(&operation);
    let matched_manifests = get_matches_tum(&tum_manifests, &operation);
    // dbg!(&matched_manifests);
    let security_count = matched_manifests
        .iter()
        .filter(|(_, entry_ref)| entry_ref.is_security())
        .count();

    Ok(security_count)
}

pub async fn check_security_upgradable_detail(apt: &OmaApt) -> Result<Vec<SecurityUpdateInfo>> {
    let sysroot = std::path::Path::new("/");
    let tum_manifests = get_tum(sysroot)?;

    let operation = check_upgradable_detail(apt).await?;
    let matched_manifests = get_matches_tum(&tum_manifests, &operation);

    let security_updates: Vec<SecurityUpdateInfo> = matched_manifests
        .into_iter()
        .filter(|(_, entry_ref)| entry_ref.is_security())
        .map(|(manifest_name, entry_ref)| {
            let package_names = match entry_ref {
                oma_tum::TopicUpdateEntryRef::Conventional { packages, .. } => {
                    packages.keys().map(|name| name.to_string()).collect()
                }
                oma_tum::TopicUpdateEntryRef::Cumulative { .. } => {
                    // Cumulative has no packages info
                    // Returns an empty vector
                    Vec::new()
                }
            };

            SecurityUpdateInfo {
                manifest_name: manifest_name.to_string(),
                is_security: entry_ref.is_security(),
                package_count: entry_ref.count_packages(),
                package_names,
            }
        })
        .collect();

    Ok(security_updates)
}

#[tokio::test]
async fn test_check_upgradable_detail() {
    let apt = OmaApt::new(
        vec![],
        OmaAptArgs::builder().build(),
        false,
        oma_pm::apt::AptConfig::new(),
    )
    .unwrap();
    let result = check_upgradable_detail(&apt).await.unwrap();
    dbg!(result);
}

#[tokio::test]
async fn test_check_security_upgradable() {
    let apt = OmaApt::new(
        vec![],
        OmaAptArgs::builder().build(),
        false,
        oma_pm::apt::AptConfig::new(),
    )
    .unwrap();
    let result = check_security_upgradable(&apt).await.unwrap();
    dbg!(format!("Total Security Upgradable: {result}"));
}

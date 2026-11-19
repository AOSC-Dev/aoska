use super::{amo_types::PmPackage, index::Index, packages::PackageBrief};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

pub fn preferred_desktop_id<'a>(
    index: Option<&'a str>,
    detail: Option<&'a str>,
) -> Option<&'a str> {
    index.or(detail)
}

pub fn unique_entries(index: Index) -> Vec<PackageBrief> {
    let mut names = HashSet::new();
    index
        .packages
        .into_iter()
        .flat_map(|category| category.packages)
        .filter(|entry| names.insert(entry.name.clone()))
        .collect()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogApplication {
    #[serde(flatten)]
    pub package: PmPackage,
    pub catalog_name: String,
    pub icon: String,
    pub desktop_id: Option<String>,
}

pub fn installed_entries(
    entries: Vec<PackageBrief>,
    states: Vec<PmPackage>,
    base_url: &str,
) -> anyhow::Result<Vec<CatalogApplication>> {
    let mut states: HashMap<_, _> = states
        .into_iter()
        .map(|state| (state.name.clone(), state))
        .collect();
    let base = reqwest::Url::parse(&format!("{}/", base_url.trim_end_matches('/')))?;
    entries
        .into_iter()
        .filter_map(|entry| {
            states
                .remove(&entry.name)
                .filter(|state| state.installed && state.aoska_manageable)
                .map(|package| {
                    let path = entry.icon.to_string_lossy();
                    let icon = if path.starts_with("https://")
                        || path.starts_with("http://")
                        || path.starts_with("data:")
                    {
                        path.into_owned()
                    } else {
                        base.join(&format!("packages/{}/{}", entry.name, path))?
                            .to_string()
                    };
                    Ok(CatalogApplication {
                        package,
                        catalog_name: entry.name,
                        icon,
                        desktop_id: entry.desktop_id,
                    })
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_are_joined_once_and_old_catalogs_remain_valid() {
        let index: Index = serde_json::from_value(serde_json::json!({
            "version":1,"generated_at":"2026-11-17T00:00:00Z",
            "packages":[
                {"category":"working","packages":[{"name":"firefox","intro":"Browser","icon":"assets/logo.png"}]},
                {"category":"creating","packages":[{"name":"firefox","intro":"Browser","icon":"assets/logo.png"}]}
            ]
        })).unwrap();
        let entries = unique_entries(index);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].desktop_id.is_none());
        let state = PmPackage {
            name: "firefox".into(),
            display_name: "firefox".into(),
            package_name: "firefox:amd64".into(),
            installed: true,
            upgradable: false,
            openable: false,
            aoska_manageable: true,
            requires_manual_action: false,
            current_version: Some("159".into()),
            new_version: Some("159".into()),
            installed_size: None,
            download_size: None,
            architecture: Some("amd64".into()),
            is_security: false,
        };
        let rows =
            installed_entries(entries.clone(), vec![state], "http://127.0.0.1:1234").unwrap();
        assert_eq!(rows[0].package.package_name, "firefox:amd64");
        assert_eq!(
            rows[0].icon,
            "http://127.0.0.1:1234/packages/firefox/assets/logo.png"
        );
        assert!(installed_entries(entries, vec![], "http://127.0.0.1")
            .unwrap()
            .is_empty());
    }

    #[test]
    fn explicit_catalog_ids_precede_the_package_name_fallback() {
        assert_eq!(
            preferred_desktop_id(None, Some("org.example.Editor.desktop")),
            Some("org.example.Editor.desktop")
        );
        assert_eq!(
            preferred_desktop_id(Some("chosen.desktop"), Some("old.desktop")),
            Some("chosen.desktop")
        );
        assert_eq!(preferred_desktop_id(None, None), None);
    }
}

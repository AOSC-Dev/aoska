use super::amo_types::PmPackage;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

#[derive(Deserialize)]
pub struct CatalogIndex {
    pub schema_version: u32,
    apps: Vec<CatalogEntry>,
}

#[derive(Deserialize)]
pub struct CatalogEntry {
    pub id: String,
    pub package_refs: Vec<String>,
    pub desktop_id: Option<String>,
    icon: Option<String>,
    external: bool,
    installer_kinds: Vec<String>,
}

impl CatalogIndex {
    // For mixed installers the index includes expected names of external artifacts too.
    pub fn mixed_ids(&self) -> Vec<String> {
        self.apps
            .iter()
            .filter(|app| !app.external && app.mixed_installers())
            .map(|app| app.id.clone())
            .collect()
    }

    pub fn set_repository_refs(&mut self, id: &str, names: Vec<String>) {
        if let Some(app) = self.apps.iter_mut().find(|app| app.id == id) {
            app.package_refs = names;
            app.installer_kinds.retain(|kind| kind == "aosc-repository");
        }
    }
}

impl CatalogEntry {
    fn mixed_installers(&self) -> bool {
        self.installer_kinds
            .iter()
            .any(|kind| kind == "aosc-repository")
            && self
                .installer_kinds
                .iter()
                .any(|kind| kind != "aosc-repository")
    }

    pub fn launch_candidates(&self, selected: Option<&str>) -> anyhow::Result<Vec<String>> {
        match selected {
            Some(name) if self.package_refs.iter().any(|package| package == name) => {
                Ok(vec![name.to_owned()])
            }
            Some(_) => anyhow::bail!("Package does not belong to this catalog application"),
            None => Ok(self.package_refs.clone()),
        }
    }
}

pub fn repository_entries(index: CatalogIndex) -> Vec<CatalogEntry> {
    index
        .apps
        .into_iter()
        .filter(|app| {
            !app.external
                && !app.package_refs.is_empty()
                && !app.installer_kinds.is_empty()
                && app
                    .installer_kinds
                    .iter()
                    .all(|kind| kind == "aosc-repository")
        })
        .collect()
}

pub fn package_names(entries: &[CatalogEntry]) -> Vec<String> {
    entries
        .iter()
        .flat_map(|app| app.package_refs.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
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
    entries: Vec<CatalogEntry>,
    states: Vec<PmPackage>,
    base_url: &str,
) -> anyhow::Result<Vec<CatalogApplication>> {
    let mut installed: HashMap<_, _> = states
        .into_iter()
        .filter(|state| state.installed && state.aoska_manageable)
        .map(|state| (state.name.clone(), state))
        .collect();
    let base = reqwest::Url::parse(&format!("{}/", base_url.trim_end_matches('/')))?;
    let mut applications = Vec::new();
    for app in entries {
        for name in &app.package_refs {
            if let Some(package) = installed.remove(name) {
                let icon = app.icon.as_ref().map(|path| base.join(path)).transpose()?;
                applications.push(CatalogApplication {
                    package,
                    catalog_name: app.id.clone(),
                    icon: icon.map(String::from).unwrap_or_default(),
                    desktop_id: app.desktop_id.clone(),
                });
            }
        }
    }
    Ok(applications)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn index() -> CatalogIndex {
        serde_json::from_value(json!({
            "schema_version": 1,
            "apps": [
                {"id":"sample-repository-app","package_refs":["sample-app","sample-extra"],"icon":"apps/sample-repository-app/assets/icon.svg","desktop_id":"org.example.Sample.desktop","external":false,"installer_kinds":["aosc-repository"]},
                {"id":"external-app","package_refs":["external-deb"],"external":true,"installer_kinds":["browser-assisted-deb"]},
                {"id":"mixed","package_refs":["upstream-deb","repo-pkg"],"external":false,"installer_kinds":["direct-deb","aosc-repository"]}
            ]
        })).unwrap()
    }

    #[test]
    fn repository_refs_exclude_external_artifacts_and_respect_package_selection() {
        let mut index = index();
        assert_eq!(index.mixed_ids(), ["mixed"]);
        index.set_repository_refs("mixed", vec!["repo-pkg".into()]);
        let entries = repository_entries(index);
        assert_eq!(
            package_names(&entries),
            ["repo-pkg", "sample-app", "sample-extra"]
        );
        let app = &entries[0];
        assert_eq!(
            app.launch_candidates(Some("sample-extra")).unwrap(),
            ["sample-extra"]
        );
        assert!(app.launch_candidates(Some("upstream-deb")).is_err());
        assert_eq!(app.launch_candidates(None).unwrap(), app.package_refs);
    }

    #[test]
    fn installed_rows_preserve_amo_package_keys_and_link_to_the_catalog_app() {
        let state = PmPackage {
            name: "sample-app".into(),
            display_name: "Sample".into(),
            package_name: "sample-app:amd64".into(),
            installed: true,
            upgradable: false,
            openable: false,
            aoska_manageable: true,
            requires_manual_action: false,
            current_version: Some("1".into()),
            new_version: Some("1".into()),
            installed_size: None,
            download_size: None,
            architecture: Some("amd64".into()),
            is_security: false,
        };
        let rows = installed_entries(
            repository_entries(index()),
            vec![state],
            "http://127.0.0.1:8765/dist/v1",
        )
        .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].package.package_name, "sample-app:amd64");
        assert_eq!(rows[0].catalog_name, "sample-repository-app");
        assert_eq!(
            rows[0].desktop_id.as_deref(),
            Some("org.example.Sample.desktop")
        );
        assert_eq!(
            rows[0].icon,
            "http://127.0.0.1:8765/dist/v1/apps/sample-repository-app/assets/icon.svg"
        );
    }
}

use anyhow::{ensure, Context, Result};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::PathBuf};

pub const LOCAL_REPO_PATH: &str = "/opt/aoska/";
const DEV_URL: &str = "https://aosc-dev.github.io/aosc-os-asmr/v1/";
const STABLE_URL: &str = "https://repo.aosc.io/asmr/v1/";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "channel", rename_all = "lowercase")]
pub enum CatalogSelection {
    #[default]
    Auto,
    Dev,
    Stable,
    Custom {
        #[serde(rename = "customUrl")]
        custom_url: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogSettings {
    selection: CatalogSelection,
    default_channel: &'static str,
    pub effective_url: String,
    overridden: bool,
}

fn directory_url(value: &str) -> Result<String> {
    let url = Url::parse(value.trim()).context("Invalid ASMR URL")?;
    ensure!(
        matches!(url.scheme(), "http" | "https")
            && url.has_host()
            && url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none(),
        "ASMR requires an HTTP(S) directory URL without credentials, a query or a fragment"
    );
    Ok(if url.path().ends_with('/') {
        url.to_string()
    } else {
        format!("{url}/")
    })
}

pub struct CatalogPreferences {
    path: PathBuf,
    override_url: Option<String>,
}

impl CatalogPreferences {
    pub fn new(config_dir: PathBuf, override_url: Option<String>) -> Self {
        Self {
            path: config_dir.join("catalog-source.json"),
            override_url: override_url.filter(|value| !value.trim().is_empty()),
        }
    }

    fn resolve(&self, mut selection: CatalogSelection) -> Result<CatalogSettings> {
        let dev_build = cfg!(debug_assertions);
        let selected_url = match &mut selection {
            CatalogSelection::Auto if dev_build => DEV_URL,
            CatalogSelection::Auto | CatalogSelection::Stable => STABLE_URL,
            CatalogSelection::Dev => DEV_URL,
            CatalogSelection::Custom { custom_url } => {
                *custom_url = directory_url(custom_url)?;
                custom_url.as_str()
            }
        };
        let effective_url = match self.override_url.as_deref() {
            Some(value) => directory_url(value).context("Invalid AOSKA_ASMR_URL")?,
            None => selected_url.to_owned(),
        };
        Ok(CatalogSettings {
            selection,
            default_channel: if dev_build { "dev" } else { "stable" },
            effective_url,
            overridden: self.override_url.is_some(),
        })
    }

    pub fn read(&self) -> Result<CatalogSettings> {
        let selection = match fs::read(&self.path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .with_context(|| format!("Invalid catalog settings in {}", self.path.display()))?,
            Err(error) if error.kind() == ErrorKind::NotFound => CatalogSelection::Auto,
            Err(error) => {
                return Err(error).with_context(|| format!("Cannot read {}", self.path.display()))
            }
        };
        self.resolve(selection)
    }

    pub fn save(&self, selection: CatalogSelection) -> Result<CatalogSettings> {
        // Preserve an unreadable or malformed settings file for recovery.
        self.read()?;
        let settings = self.resolve(selection)?;
        fs::create_dir_all(self.path.parent().unwrap())?;
        let pending = self
            .path
            .with_extension(format!("{}.tmp", std::process::id()));
        fs::write(&pending, serde_json::to_vec_pretty(&settings.selection)?)
            .with_context(|| format!("Cannot write {}", pending.display()))?;
        fs::rename(&pending, &self.path)
            .with_context(|| format!("Cannot replace {}", self.path.display()))?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_builtin_channels_and_environment_override() {
        let mut preferences = CatalogPreferences::new(PathBuf::new(), Some("  ".into()));
        let automatic = preferences.resolve(CatalogSelection::Auto).unwrap();
        assert_eq!(
            automatic.default_channel,
            if cfg!(debug_assertions) {
                "dev"
            } else {
                "stable"
            }
        );
        assert_eq!(
            automatic.effective_url,
            if cfg!(debug_assertions) {
                DEV_URL
            } else {
                STABLE_URL
            }
        );
        assert_eq!(
            preferences
                .resolve(CatalogSelection::Dev)
                .unwrap()
                .effective_url,
            DEV_URL
        );
        assert_eq!(
            preferences
                .resolve(CatalogSelection::Stable)
                .unwrap()
                .effective_url,
            STABLE_URL
        );
        assert!(!automatic.overridden);
        preferences.override_url = Some("http://127.0.0.1:8765/examples/v1".into());
        let overridden = preferences.resolve(CatalogSelection::Stable).unwrap();
        assert!(overridden.overridden);
        assert_eq!(
            overridden.effective_url,
            "http://127.0.0.1:8765/examples/v1/"
        );
        assert_eq!(overridden.selection, CatalogSelection::Stable);
    }

    #[test]
    fn rejects_invalid_directory_urls() {
        for invalid in [
            "",
            "v1/",
            "file:///tmp/catalog",
            "https://server/path?q=1",
            "https://server/#file",
            "https://name:password@server/v1",
        ] {
            assert!(directory_url(invalid).is_err(), "{invalid}");
        }
    }

    #[test]
    fn saves_and_reloads_the_selection_and_preserves_corrupt_settings() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory =
            std::env::temp_dir().join(format!("aoska-settings-{}-{unique}", std::process::id()));
        let preferences = CatalogPreferences::new(directory.clone(), None);
        assert_eq!(
            preferences.read().unwrap().selection,
            CatalogSelection::Auto
        );
        let custom = CatalogSelection::Custom {
            custom_url: " https://example.org/v1 ".into(),
        };
        assert_eq!(
            preferences.save(custom).unwrap().effective_url,
            "https://example.org/v1/"
        );
        let reopened = CatalogPreferences::new(directory.clone(), None);
        assert_eq!(
            reopened.read().unwrap().effective_url,
            "https://example.org/v1/"
        );
        let invalid = CatalogSelection::Custom {
            custom_url: "file:///tmp".into(),
        };
        assert!(reopened.save(invalid).is_err());
        assert_eq!(
            reopened.read().unwrap().effective_url,
            "https://example.org/v1/"
        );
        reopened.save(CatalogSelection::Stable).unwrap();
        assert_eq!(preferences.read().unwrap().effective_url, STABLE_URL);
        fs::write(&preferences.path, b"{").unwrap();
        assert!(preferences.read().is_err());
        assert!(preferences.save(CatalogSelection::Dev).is_err());
        assert_eq!(fs::read(&preferences.path).unwrap(), b"{");
        fs::remove_dir_all(directory).unwrap();
    }
}

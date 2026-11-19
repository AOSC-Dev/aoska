use anyhow::{bail, Context, Result};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopApplication {
    pub id: String,
    pub name: String,
}

fn entry_id(package_name: &str, desktop_id: Option<&str>) -> Result<String> {
    let id = desktop_id
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{package_name}.desktop"));
    if !id.ends_with(".desktop")
        || id == ".desktop"
        || id.contains("..")
        || id.contains(['/', '\\'])
        || id.chars().any(char::is_control)
    {
        bail!("Invalid desktop application ID");
    }
    Ok(id)
}

#[cfg(target_os = "linux")]
fn visible_application(info: &gio::DesktopAppInfo) -> bool {
    use gio::prelude::*;
    !info.is_hidden()
        && !info.is_nodisplay()
        && info.should_show()
        && (!info.executable().as_os_str().is_empty() || info.boolean("DBusActivatable"))
}

pub fn resolve(package_name: &str, desktop_id: Option<&str>) -> Result<Option<DesktopApplication>> {
    let id = entry_id(package_name, desktop_id)?;
    #[cfg(target_os = "linux")]
    {
        use gio::prelude::*;
        Ok(gio::DesktopAppInfo::new(&id)
            .filter(visible_application)
            .map(|info| DesktopApplication {
                id,
                name: info.display_name().to_string(),
            }))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = id;
        Ok(None)
    }
}

pub fn launch(package_name: &str, desktop_id: Option<&str>) -> Result<()> {
    let id = entry_id(package_name, desktop_id)?;
    #[cfg(target_os = "linux")]
    {
        use gio::prelude::*;
        let info = gio::DesktopAppInfo::new(&id)
            .filter(visible_application)
            .with_context(|| format!("No available desktop application: {id}"))?;
        info.launch(&[], None::<&gio::AppLaunchContext>)
            .with_context(|| format!("Could not launch {id}"))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = id;
        bail!("Desktop application launching is only supported on Linux")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_ids_and_package_fallback_are_validated() {
        assert_eq!(entry_id("vlc", None).unwrap(), "vlc.desktop");
        assert_eq!(
            entry_id("firefox", Some("org.mozilla.firefox.desktop")).unwrap(),
            "org.mozilla.firefox.desktop"
        );
        for id in [
            "/tmp/app.desktop",
            "../app.desktop",
            "a/b.desktop",
            "a\\b.desktop",
            ".desktop",
            "app",
        ] {
            assert!(entry_id("vlc", Some(id)).is_err());
        }
        assert!(resolve("aoska-nonexistent-test-application-271936", None)
            .unwrap()
            .is_none());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn gio_validates_visibility_and_parses_quoted_exec() {
        use gio::prelude::*;
        let directory =
            std::env::temp_dir().join(format!("aoska-desktop-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("test.desktop");
        for (extra, visible) in [
            ("", true),
            ("Hidden=true\n", false),
            ("NoDisplay=true\n", false),
        ] {
            std::fs::write(&path, format!("[Desktop Entry]\nType=Application\nName=Application With Spaces\nExec=/usr/bin/true \"argument with spaces\" %U\n{extra}")).unwrap();
            let info = gio::DesktopAppInfo::from_filename(&path).unwrap();
            assert_eq!(visible_application(&info), visible);
            assert_eq!(info.display_name(), "Application With Spaces");
            assert_eq!(info.executable().to_str(), Some("/usr/bin/true"));
        }
        std::fs::write(&path, "[Desktop Entry]\nType=Application\nName=Missing executable\nExec=/aoska/no-such-program\n").unwrap();
        assert!(gio::DesktopAppInfo::from_filename(&path).is_none());
        std::fs::remove_dir_all(directory).unwrap();
    }
}

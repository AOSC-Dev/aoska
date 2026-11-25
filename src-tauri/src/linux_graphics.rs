fn needs_explicit_sync_workaround(
    wayland_available: bool,
    gdk_backend: Option<&str>,
    nvidia_loaded: bool,
    explicitly_configured: bool,
) -> bool {
    if !wayland_available || !nvidia_loaded || explicitly_configured {
        return false;
    }

    // Respect GTK's preferred backend, including an explicit X11 choice.
    match gdk_backend {
        None => true,
        Some(backends) => backends
            .split(',')
            .map(str::trim)
            .next()
            .is_some_and(|backend| matches!(backend, "wayland" | "*")),
    }
}

#[cfg(target_os = "linux")]
pub fn configure() {
    use std::{env, path::Path};

    let wayland_available = env::var_os("WAYLAND_DISPLAY").is_some_and(|value| !value.is_empty());
    let gdk_backend = env::var("GDK_BACKEND").ok();
    let nvidia_loaded = Path::new("/sys/module/nvidia").is_dir();
    let explicitly_configured = env::var_os("__NV_DISABLE_EXPLICIT_SYNC").is_some();

    if needs_explicit_sync_workaround(
        wayland_available,
        gdk_backend.as_deref(),
        nvidia_loaded,
        explicitly_configured,
    ) {
        // This runs in main before GTK, WebKit, or background threads start.
        // egl-wayland otherwise enables syncobj on a GTK-managed surface whose
        // SHM commits omit an acquire point. Keep WebKit's DMA-BUF path enabled.
        // https://bugs.webkit.org/show_bug.cgi?id=280210
        env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
        eprintln!("Aoska: NVIDIA Wayland compatibility enabled (__NV_DISABLE_EXPLICIT_SYNC=1)");
    }
}

#[cfg(test)]
mod tests {
    use super::needs_explicit_sync_workaround;

    #[test]
    fn applies_to_nvidia_wayland_with_default_or_prioritized_backend() {
        for backend in [None, Some("wayland"), Some("wayland,x11"), Some("*")] {
            assert!(needs_explicit_sync_workaround(true, backend, true, false));
        }
    }

    #[test]
    fn preserves_explicit_x11_choice_and_other_drivers() {
        for backend in [Some("x11"), Some("x11,wayland"), Some("broadway")] {
            assert!(!needs_explicit_sync_workaround(true, backend, true, false));
        }
        assert!(!needs_explicit_sync_workaround(false, None, true, false));
        assert!(!needs_explicit_sync_workaround(true, None, false, false));
    }

    #[test]
    fn preserves_explicit_user_configuration() {
        assert!(!needs_explicit_sync_workaround(true, None, true, true));
    }
}

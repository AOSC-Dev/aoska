use gtk::prelude::*;
use tauri::{Manager, Runtime, WebviewWindowBuilder};

pub fn build<R: Runtime, M: Manager<R>>(
    builder: WebviewWindowBuilder<'_, R, M>,
) -> anyhow::Result<()> {
    let app = gio::Application::default()
        .and_then(|app| app.downcast::<gtk::Application>().ok())
        .ok_or_else(|| anyhow::anyhow!("GTK application unavailable"))?;
    let screen =
        gtk::gdk::Screen::default().ok_or_else(|| anyhow::anyhow!("GTK screen unavailable"))?;
    let style = gtk::CssProvider::new();
    style.load_from_data(include_bytes!("window_frame.css"))?;
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &style,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // GTK needs the custom title bar before realization. Aoska's navigation bar
    // already supplies the title and window controls inside the opaque WebView.
    let handler = app.connect_window_added(|_, window| {
        window.set_widget_name("aoska-window");
        window.set_titlebar(Some(&gtk::Box::new(gtk::Orientation::Horizontal, 0)));
    });
    let window = builder.decorations(true).visible(false).build();
    app.disconnect(handler);
    let window = window?;
    // Tao supplies its own title bar on Wayland. Hide whichever widget GTK selected.
    if let Some(titlebar) = window.gtk_window()?.titlebar() {
        titlebar.set_no_show_all(true);
        titlebar.hide();
    }
    window.default_vbox()?.set_widget_name("aoska-content");
    window.show()?;
    Ok(())
}

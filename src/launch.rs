use gtk::gio;
use gtk::prelude::*;
use std::path::Path;

/// A launch context carrying our own activation token (Wayland), or GDK's
/// launch context when there's no token (X11, no window, no xdg-activation).
pub fn launch_context(window: Option<&gtk::ApplicationWindow>) -> Option<gio::AppLaunchContext> {
    match window.and_then(crate::token) {
        Some(token) => {
            let ctx = gio::AppLaunchContext::new();
            ctx.setenv("XDG_ACTIVATION_TOKEN", &token);
            ctx.setenv("DESKTOP_STARTUP_ID", &token);
            Some(ctx)
        }
        None => gtk::gdk::Display::default()
            .and_then(|d| d.app_launch_context())
            .map(|c| c.upcast()),
    }
}

/// Opens `uri` (`obsidian://…`, `https://…`, `file://…`) with its default app.
pub fn launch_uri(window: Option<&gtk::ApplicationWindow>, uri: &str) -> Result<(), String> {
    let ctx = launch_context(window);
    gio::AppInfo::launch_default_for_uri(uri, ctx.as_ref()).map_err(|e| e.to_string())
}

pub fn open_path(window: Option<&gtk::ApplicationWindow>, path: &Path) -> Result<(), String> {
    launch_uri(window, &gio::File::for_path(path).uri())
}

//! Launching another app from a GTK window (Tauri on Linux) so that it comes
//! to the front on GNOME/Wayland instead of opening behind the caller.
//!
//! Call [`init`] once on the GTK main thread after GTK is up (Tauri's
//! `setup`), then [`launch_uri`] / [`open_path`] on the main thread while the
//! window still has focus. On X11 or without xdg-activation it falls back to
//! GDK's own launch context. Everything is Linux-only; on other platforms the
//! crate is empty and callers use their usual opener.

#[cfg(target_os = "linux")]
mod activation;
#[cfg(target_os = "linux")]
mod launch;

#[cfg(target_os = "linux")]
pub use activation::{init, token};
#[cfg(target_os = "linux")]
pub use launch::{launch_context, launch_uri, open_path};

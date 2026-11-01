# wayland-launch

[![CI](https://github.com/mwngrn/wayland-launch/actions/workflows/ci.yml/badge.svg)](https://github.com/mwngrn/wayland-launch/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

When a Tauri (GTK) app on GNOME/Wayland opens another app (Obsidian via `obsidian://`, a browser, a file), GNOME only lets the new app take focus if the launch carries an xdg-activation token tied to the user's latest input. Without one, the opened app ends up behind the caller. This crate gets such a token from the caller's own keyboard and pointer events and launches through GIO with it.

## Use

```toml
[target.'cfg(target_os = "linux")'.dependencies]
wayland-launch = { git = "https://github.com/mwngrn/wayland-launch", tag = "v0.1.1" }
```

```rust
// In Tauri's setup (GTK main thread):
#[cfg(target_os = "linux")]
wayland_launch::init();

// Later, on the main thread, while the window has focus:
let gtk = window.gtk_window().ok();
wayland_launch::launch_uri(gtk.as_ref(), "obsidian://open?path=…")?;
```

`launch_context` is public too, for launching a specific app (`gio::DesktopAppInfo`) instead of the default handler.

To work on the crate and an app at the same time, point the app at the local checkout with a gitignored `[patch]` in its `.cargo/config.toml`:

```toml
[patch."https://github.com/mwngrn/wayland-launch"]
wayland-launch = { path = "../wayland-launch" }
```

## Releasing

Bump `version` in `Cargo.toml`, commit, tag `vX.Y.Z`, push the tag, then bump the tag in each app.

## License

MIT, see [LICENSE](LICENSE).

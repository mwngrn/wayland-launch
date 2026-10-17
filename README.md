# wayland-launch

When a Tauri (GTK) app on GNOME/Wayland opens another app (Obsidian via `obsidian://`, a browser, a file), GNOME only lets the new app take focus if the launch carries an xdg-activation token tied to the user's latest input. Without one, the opened app ends up behind the caller. This crate gets such a token from the caller's own keyboard and pointer events and launches through GIO with it.

## Use

```toml
[target.'cfg(target_os = "linux")'.dependencies]
wayland-launch = { git = "ssh://git@github.com/mwngrn/wayland-launch.git", tag = "v0.1.0" }
```

The repo is private, so cargo has to fetch with the system git (which has the SSH key). Add this to the consuming repo's `.cargo/config.toml`:

```toml
[net]
git-fetch-with-cli = true
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
[patch."ssh://git@github.com/mwngrn/wayland-launch.git"]
wayland-launch = { path = "../wayland-launch" }
```

## Releasing

Bump `version` in `Cargo.toml`, commit, tag `vX.Y.Z`, push the tag, then bump the tag in each app.

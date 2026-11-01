Level: base
Visibility: public (MIT)

# wayland-launch

A small Rust crate (GTK/Wayland, no Tauri) that lets a Tauri app open another app in the foreground on GNOME/Wayland. Used by skriva, m2d and kommando. Why it is a repo of its own: `docs/decisions/adr-001-own-repo-git-dependency.md`.

## Commands

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings   # same as CI
```

No unit tests: everything here talks to a real Wayland session and GDK. Verification happens in the apps (does Obsidian open in front?).

## Release

Consumers lock a tag (`vX.Y.Z`). To release a change: bump `version` in `Cargo.toml`, tag, push the tag, then bump the tag in the apps.

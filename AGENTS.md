Nivå: bas
Synlighet: privat

# wayland-launch

Liten Rust-crate (GTK/Wayland, ingen Tauri) som låter en Tauri-app öppna en annan app i förgrunden på GNOME/Wayland. Används av skriva och m2d. kommando har en egen kopia som ska bytas mot den här. Arbetssätt: [[dev-wow]]. Varför ett eget repo: `docs/decisions/adr-001-own-repo-git-dependency.md`.

## Kommandon

```bash
cargo clippy --all-targets
```

Inga enhetstester: allt här pratar med en riktig Wayland-session och GDK. Verifiering sker i apparna (öppnas Obsidian framför?).

## Release

Konsumenterna låser en tagg (`vX.Y.Z`). Ändring → bumpa version, tagga, pusha taggen, bumpa taggen i apparna.

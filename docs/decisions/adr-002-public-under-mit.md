# Design Choice: Public under MIT

## Status
**Accepted**. 2026-11-01.

## Context
As a private repo, every consumer needed an SSH URL and `net.git-fetch-with-cli = true`, and every build machine needed a key. That stopped GitHub Actions and cloud agents from building the apps that depend on it (skriva, kommando, m2d) without a deploy key per app. The crate holds no secrets and nothing specific to one product.

## Decision
Make the repo public under the MIT license, with its history kept. Consumers use `https://github.com/mwngrn/wayland-launch` with a tag.

## Rationale
- No keys to hand out or rotate for CI and agents.
- The full history was scanned (gitleaks, trufflehog and a manual pass) before publishing, and nothing sensitive was found.
- Small, general glue code that others on GNOME/Wayland may find useful.

## Consequences
- CI (`cargo fmt --check`, `cargo clippy -D warnings`) runs on every PR and push to `main`.
- The consumers' `Cargo.toml` and `Cargo.lock` move from the SSH URL to the https URL.

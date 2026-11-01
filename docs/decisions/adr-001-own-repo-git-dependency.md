# Design Choice: An own repo, used as a cargo git dependency

## Status
**Accepted**

## Context
kommando and m2d each had a copy of the same Wayland focus code, and skriva needed it too, so it was time to break it out. The established pattern for shared Rust code is a crate in the product repo that owns the format (m2d-core in m2d). This code has no format and no owning product: it's desktop glue for any Tauri app on Linux.

## Decision
A small repo of its own, `mwngrn/wayland-launch`, depended on as a cargo git dependency locked to a tag. It isn't a git submodule and doesn't live inside one of the apps.

## Rationale
- A git dependency is just a `Cargo.toml` line. Cargo fetches and caches it, and the apps' own checkouts don't change. A submodule has to be initialised and updated in every clone, and pins by commit without a version.
- Living in kommando would make m2d and skriva fetch all of kommando for 250 lines, and tie the crate's releases to kommando's.
- Tags give each app control over when it takes a change.

## Consequences
- Changing the crate means tag and bump in each app. For simultaneous work, a gitignored `[patch]` points at the sibling checkout (see README).
- The repo started out private, which meant SSH URLs and `net.git-fetch-with-cli` in every consumer. ADR-002 made it public.

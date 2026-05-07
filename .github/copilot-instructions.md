# Copilot Instructions — starship-manager

## Repository overview

`starship-manager` is a cross-platform Rust TUI application for managing
[Starship](https://starship.rs/) prompt themes/presets. It is organised as a
Cargo workspace with four internal crates plus a thin binary entrypoint.

## Workspace layout

```
crates/
  core/       # profiles, theme bundles, config IO, TOML validation
  preview/    # fixture env, spawn `starship prompt`, ANSI capture
  tui/        # ratatui-based UI (three-pane layout, event loop, modals)
  install/    # installer/update provider abstraction (winget, brew, script)
src/
  main.rs     # binary — sets up terminal and runs the TUI event loop
examples/     # sample Starship TOML presets
docs/         # architecture, preview engine, and theme bundle docs
```

## Conventions

* **Edition:** Rust 2024 (`edition = "2024"`).
* **Error handling:** Use `anyhow::Result` for fallible functions.
* **Serialization:** `serde` + `serde_json` for bundles; `toml_edit` for
  comment-preserving TOML manipulation.
* **TUI framework:** `ratatui` + `crossterm`.
* **Tests:** Unit tests live in the same file (`#[cfg(test)] mod tests`).
  Integration tests go in `tests/` directories at the crate root.
* **CI:** GitHub Actions on Win/mac/Linux — `cargo fmt --check`, `cargo clippy
  -D warnings`, `cargo test`.

## Adding a feature

1. Decide which crate owns the feature (`core`, `preview`, `tui`, or `install`).
2. Write the implementation with unit tests.
3. If UI changes are needed, update `crates/tui/src/ui.rs` and `event.rs`.
4. Run `cargo fmt`, `cargo clippy --workspace -- -D warnings`, and `cargo test
   --workspace` locally.
5. Update docs in `docs/` if the feature touches architecture or data formats.

## Adding a new install provider

1. Add a variant to `InstallProvider` in `crates/install/src/provider.rs`.
2. Add detection logic in `detect_provider()`.
3. Add the command invocation in `install_or_update()`.
4. Document the provider in `docs/architecture.md`.

## Running locally

```sh
cargo run          # launch the TUI
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

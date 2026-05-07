# starship-manager

A cross-platform (Windows / macOS / Linux) Rust TUI application for managing
[Starship](https://starship.rs/) prompt themes, presets, and configuration.

## Features

- **Profile management** — import, export, and switch between Starship
  configurations stored as `.toml` files.
- **Structured + raw TOML editing** — edit configs directly in the TUI with
  TOML validation.
- **Real prompt preview** — invokes the local `starship` binary with a
  temporary config and fixture environment, capturing and displaying the actual
  prompt output.
- **Theme bundles** — shareable JSON bundles with metadata (name, author,
  description, version) and embedded TOML config.
- **Starship installer** — detect and run the appropriate install/update
  command (winget on Windows, brew on macOS/Linux, official install script
  fallback).

## Quick start

```sh
# Clone and run
git clone https://github.com/sytone/starship-manager.git
cd starship-manager
cargo run

# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings
cargo fmt --all --check
```

## Layout

| Key         | Action                          |
|-------------|---------------------------------|
| `q`         | Quit                            |
| `Tab`       | Cycle pane focus                |
| `↑` / `↓`  | Navigate list or editor         |
| `s`         | Save current profile            |
| `p`         | Refresh preview                 |
| `a`         | Apply profile to starship.toml  |
| `i`         | Install/update starship         |
| `?`         | Show help                       |

## Workspace structure

```
crates/
  core/       — profiles, bundles, config IO, validation
  preview/    — fixture env, starship invocation, ANSI capture
  tui/        — ratatui UI (three-pane layout, events, modals)
  install/    — install/update provider abstraction
src/main.rs   — binary entrypoint
examples/     — sample Starship presets
docs/         — architecture and design docs
```

## License

[MIT](LICENSE)

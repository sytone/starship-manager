# Architecture

## Overview

`starship-manager` is a cross-platform Rust TUI that manages Starship prompt
themes. It is structured as a Cargo workspace with four library crates and one
binary crate.

## Crate map

```
┌──────────────────────────────────────────────────────────┐
│  src/main.rs  (binary)                                   │
│  Sets up terminal, creates App, runs event loop          │
└────────────────────────┬─────────────────────────────────┘
                         │
          ┌──────────────▼──────────────┐
          │  crates/tui                  │
          │  UI layout, widgets, events  │
          └──┬──────────┬───────────┬───┘
             │          │           │
   ┌─────────▼──┐  ┌────▼─────┐  ┌─▼────────────┐
   │ crates/core │  │  preview │  │   install     │
   │ profiles,   │  │  fixture │  │   provider    │
   │ bundles,    │  │  spawn   │  │   abstraction │
   │ config IO   │  │  starship│  │   (winget,    │
   │ validation  │  │  ANSI    │  │    brew,      │
   └─────────────┘  └──────────┘  │    script)    │
                                  └───────────────┘
```

## Data flow

1. **Startup:** `core` reads profiles from `~/.config/starship-manager/profiles/`.
2. **Selection:** User picks a profile in the Profiles pane → editor loads its TOML.
3. **Editing:** User edits TOML in the Editor pane (basic line-level editing).
4. **Preview:** `preview` writes TOML to a temp file, spawns `starship prompt`
   with `STARSHIP_CONFIG` pointing at it, captures ANSI output, strips escape
   codes, and renders in the Preview pane.
5. **Save:** `core` writes the editor content back to the profile file.
6. **Apply:** `core::config` copies the profile content to the active
   `starship.toml` location.
7. **Install:** `install` detects the platform provider and runs the appropriate
   install command, showing output in a modal.

## Module boundaries

| Crate     | Depends on         | Responsibility                          |
|-----------|--------------------|-----------------------------------------|
| `core`    | (none)             | Profiles, bundles, config IO, validation|
| `preview` | `core`             | Fixture env, spawn starship, ANSI parse |
| `install` | (none)             | Provider detection, install/update cmds |
| `tui`     | `core`, `preview`, `install` | UI rendering, event handling   |

## Adding features

- New **data format** → `core`
- New **preview fixture** (e.g. Python virtualenv) → `preview`
- New **install provider** (e.g. scoop, pacman) → `install`
- New **UI pane or modal** → `tui`

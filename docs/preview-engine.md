# Preview Engine

## How it works

The preview engine produces a real Starship prompt output for any given TOML
configuration, without touching the user's actual shell or config.

### Steps

1. **Create fixture environment** (`PreviewEnv`):
   - A `tempfile::TempDir` is created to serve as the fake working directory.
   - Optionally, `git init` is run inside it to simulate a git repo context.
   - Additional env vars (e.g. `VIRTUAL_ENV`, `NODE_VERSION`) can be injected
     to trigger specific Starship modules.

2. **Write temporary config:**
   - The current editor TOML content is written to a file inside the temp dir
     (`starship_preview.toml`).

3. **Invoke `starship prompt`:**
   - The `starship` binary is called with:
     - `STARSHIP_CONFIG` → path to the temp config file
     - `STARSHIP_SHELL` → `bash`
     - `TERM` → `xterm-256color`
     - `PWD` → the fixture temp dir
   - stdout is captured, which contains the ANSI-escaped prompt.

4. **ANSI → display text:**
   - For the MVP, ANSI escape sequences are stripped to produce plain text.
   - **Future:** Parse SGR codes into `ratatui::text::Span` with `Style` to
     render colours faithfully in the TUI preview pane.

### Design decision: ANSI rendering

We chose to implement a minimal ANSI stripper (`strip_ansi`) rather than
pulling in an external crate because:
- The MVP only needs plain-text preview.
- A full ANSI→ratatui parser will be added in a follow-up (either via the
  `ansi-to-tui` crate or a custom parser that maps SGR 38/48 to ratatui
  `Color`).

## Fixtures

| Fixture         | Purpose                                     |
|-----------------|---------------------------------------------|
| Empty temp dir  | Baseline prompt with directory module only   |
| `git init` repo | Triggers `git_branch`, `git_status` modules  |
| Env vars        | Future: `VIRTUAL_ENV`, `NODE_VERSION`, etc.  |

## Error handling

- If `starship` is not on `PATH`, the preview pane shows an error message
  suggesting the user press `i` to install it.
- If the config TOML is invalid, `starship` will emit its own error which is
  captured and displayed.

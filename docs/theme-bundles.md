# Theme Bundles

## Overview

A theme bundle is a shareable package containing a Starship TOML configuration
plus metadata. Bundles use JSON format for easy parsing and transport.

## Format

```json
{
  "meta": {
    "name": "neon",
    "author": "username",
    "description": "A vibrant neon-coloured prompt theme",
    "version": "1.0.0"
  },
  "config_toml": "[character]\nsymbol = \"➜\"\n..."
}
```

### Fields

| Field               | Type   | Required | Description                        |
|---------------------|--------|----------|------------------------------------|
| `meta.name`         | string | yes      | Display name of the theme          |
| `meta.author`       | string | no       | Author name or handle              |
| `meta.description`  | string | no       | Short description                  |
| `meta.version`      | string | no       | SemVer version of the bundle       |
| `config_toml`       | string | yes      | Raw Starship TOML configuration    |

## Import / Export

### Exporting

```rust
use starship_manager_core::bundle::{ThemeBundle, BundleMeta};

let bundle = ThemeBundle {
    meta: BundleMeta {
        name: "my-theme".into(),
        author: "me".into(),
        description: "My custom theme".into(),
        version: "1.0.0".into(),
    },
    config_toml: std::fs::read_to_string("my-theme.toml")?,
};
bundle.export_to_file("my-theme.bundle.json".as_ref())?;
```

### Importing

```rust
let bundle = ThemeBundle::import_from_file("my-theme.bundle.json".as_ref())?;
std::fs::write("profiles/my-theme.toml", &bundle.config_toml)?;
```

## Future extensions

- **Compressed bundles:** `.tar.gz` or `.zip` containing the JSON plus
  additional assets (screenshots, README).
- **Registry:** A central index of community bundles, fetchable via URL.
- **Signing:** Optional GPG/minisign signature for bundle integrity.

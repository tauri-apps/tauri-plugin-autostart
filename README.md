# Tauri Plugin Autostart

A Tauri Plugin that allows your application to execute at startup. Supports Windows, Mac (via AppleScript or Launch Agent), and Linux.

## Installation
There are three general methods of installation that we can recommend.
1. Pull sources directly from Github using git tags / revision hashes (most secure, good for developement, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)

For more details and usage see [the example app](examples/svelte-app/src-tauri/src/main.rs).
Please note, below in the dependencies you can also lock to a revision/tag in the `Cargo.toml`.

`src-tauri/Cargo.toml`
```yaml
[dependencies]
tauri = { version = "1.0.0-rc.8" }

[dependencies.tauri-plugin-autostart]
git = "https://github.com/tauri-apps/tauri-plugin-autostart"
tag = "tauri-plugin-autostart-v0.1.0"
#branch = "main"
```

Use in `src-tauri/src/main.rs`:

```rust
use tauri_plugin_autostart::MacosLauncher;
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_autostart::init(MacosLauncher::AppleScript, false /* hidden flag */))
    .run();
}
```

# License
MIT / Apache-2.0

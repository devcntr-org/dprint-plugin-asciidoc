# dprint-plugin-asciidoc Walkthrough

I have created a dprint WASM plugin for formatting AsciiDoc files. The plugin is built with Rust using `dprint-core` and is ready for use as a WASM module.

## Changes Made

### Core Plugin Logic

- **Cargo.toml**: Configured with `dprint-core` (v0.67), `serde`, and `acdc-parser`.
- **src/lib.rs**: Implemented the `SyncPluginHandler` trait and established the WASM entry point using `generate_plugin_code!`.
- **src/configuration.rs**: Defined and implemented configuration resolution for `lineWidth`, `indentWidth`, `useTabs`, and `newLineKind`.
- **src/format.rs**: Created the formatting entry point, currently integrated with `acdc-parser` for AST verification.

### Build Pipeline

- **build.ps1**: Added a PowerShell script to compile the plugin to `wasm32-unknown-unknown` in release mode.
- **WASM binary**: Successfully generated at [target/wasm32-unknown-unknown/release/dprint_plugin_asciidoc.wasm](file:///z:/code/the-dev-center/dprint-plugin-asciidoc/target/wasm32-unknown-unknown/release/dprint_plugin_asciidoc.wasm).

## Verification Results

### WASM Compilation

The plugin successfully compiles to WASM using the latest `dprint-core` API standards.

```powershell
.\build.ps1
# Building WASM plugin...
# Build successful!
# WASM file: target/wasm32-unknown-unknown/release/dprint_plugin_asciidoc.wasm
```

### How to Use Local Plugin

To use this plugin in your [dprint.jsonc](file:///z:/code/the-dev-center/dprint-plugin-asciidoc/dprint.jsonc) file, add the local path:

```jsonc
{
  "plugins": [
    "file:///z:/code/the-dev-center/dprint-plugin-asciidoc/target/wasm32-unknown-unknown/release/dprint_plugin_asciidoc.wasm",
  ],
  "asciidoc": {
    "lineWidth": 80,
    "indentWidth": 2,
  },
}
```

> [!NOTE]
> The current version uses `acdc-parser` for structure verification. Complex formatting rules can be added to [src/format.rs](file:///z:/code/the-dev-center/dprint-plugin-asciidoc/src/format.rs) by traversing the AST.

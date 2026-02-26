<!--
  Npm Store Page README
  =====================

  This README.md is specifically designed for the npm package page at:
  https://www.npmjs.com/package/dprint-plugin-asciidoc

  Purpose:
  --------
  - Provide npm users with installation and usage instructions
  - Document the dprint plugin integration for AsciiDoc files
  - Offer a quick start guide for JavaScript/Node.js developers

  For the main project documentation and development guide, see:
  https://github.com/devcntr-app/dprint-plugin-asciidoc/blob/main/README.adoc

  Note: This file is published to npm and should remain focused on
  consumer usage, not development/build instructions.
-->

# dprint-plugin-asciidoc

[![npm version](https://badge.fury.io/js/dprint-plugin-asciidoc.svg)](https://www.npmjs.com/package/dprint-plugin-asciidoc)
[![CI](https://github.com/devcntr-app/dprint-plugin-asciidoc/actions/workflows/ci.yml/badge.svg)](https://github.com/devcntr-app/dprint-plugin-asciidoc/actions)

> An [AsciiDoc](https://asciidoc.org) formatter plugin for [dprint](https://dprint.dev)

## Installation

```bash
npm install --save-dev dprint-plugin-asciidoc
```

Or with yarn:

```bash
yarn add --dev dprint-plugin-asciidoc
```

Or with pnpm:

```bash
pnpm add --save-dev dprint-plugin-asciidoc
```

## Usage

Add the plugin to your `dprint.json` configuration:

```json
{
  "plugins": [
    "https://plugins.dprint.dev/asciidoc-0.1.0.wasm"
  ],
  "asciidoc": {
    "lineWidth": 80,
    "indentWidth": 2,
    "useTabs": false,
    "newLineKind": "lf"
  }
}
```

## Configuration Options

| Option        | Type                         | Default | Description                 |
| ------------- | ---------------------------- | ------- | --------------------------- |
| `lineWidth`   | `number`                     | `80`    | Maximum line width          |
| `indentWidth` | `number`                     | `2`     | Number of spaces per indent |
| `useTabs`     | `boolean`                    | `false` | Use tabs instead of spaces  |
| `newLineKind` | `"lf" \| "crlf" \| "system"` | `"lf"`  | Newline character style     |

## Formatting Files

After configuration, format your AsciiDoc files:

```bash
# Format all .adoc files
npx dprint fmt "**/*.adoc"

# Check formatting without writing
npx dprint check "**/*.adoc"
```

## Requirements

- [dprint](https://dprint.dev) CLI installed
- Node.js 16+ (for npm package management)

## License

MIT - see [LICENSE](https://github.com/devcntr-app/dprint-plugin-asciidoc/blob/main/LICENSE)


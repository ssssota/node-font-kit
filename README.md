# node-font-kit

Node.js wrapper for [font-kit (Rust crate)](https://crates.io/crates/font-kit).

This library works like [font-manager](https://github.com/foliojs/font-manager/).

## Features

- Get the path of installed fonts
- Get the font info
  - Full name (_Depends on platform._)
  - Family name
  - Postscript name
  - Italic(Oblique)?
  - Weight
  - Stretch
  - Monospace?

> You cannot get the correct properties with variable fonts.

> Check [this document](https://neon-bindings.com/docs/electron-apps) if you use this with Electron.

## Installation

```sh
npm install node-font-kit
```

## Support status

|                       | node14 | node16 | node18 |
| --------------------- | ------ | ------ | ------ |
| Windows x64           | ✅     | ✅     | ✅     |
| Windows x86           | ✅     | ✅     | ✅     |
| Windows arm64         | ✅     | ✅     | ✅     |
| macOS x64             | ✅     | ✅     | ✅     |
| macOS aarch64         | ✅     | ✅     | ✅     |
| Linux x64 gnu         | ✅     | ✅     | ✅     |
| Linux x64 musl        | ✅     | ✅     | ✅     |
| Linux aarch64 gnu     | ❌     | ❌     | ❌     |
| Linux aarch64 musl    | ❌     | ❌     | ❌     |
| Linux arm gnueabihf   | ❌     | ❌     | ❌     |
| Linux aarch64 android | ❌     | ❌     | ❌     |
| Linux armv7 android   | ❌     | ❌     | ❌     |
| FreeBSD x64           | ✅     | ✅     | ✅     |

[Please help to support linux-arm!](https://github.com/ssssota/node-font-kit/issues/3)

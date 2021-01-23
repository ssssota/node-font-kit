# node-font-kit

Node.js wrapper for [font-kit (Rust crate)](https://crates.io/crates/font-kit).

This library works like [font-manager](https://github.com/foliojs/font-manager/).

## Features

- [x] Get the path of installed fonts
- [ ] Get the font info
  - [ ] Full name
  - [ ] Family name
  - [ ] Postscript name
  - [ ] Italic(Opaque)?
  - [ ] Weight
  - [ ] Stretch
  - [ ] Monospace?

> Check [this document](https://neon-bindings.com/docs/electron-apps) if you use this with Electron.

## Installation

```sh
npm install node-font-kit
```

## Example

```js
const FontKit = require('node-font-kit');

// Get the path of installed fonts
FontKit.getPathAll().then(list => {
  list.forEach((path, i) => console.log(i, path));
});

/*
0 C:\\WINDOWS\\FONTS\\ROBOTO-THIN.TTF
1 C:\\WINDOWS\\FONTS\\CANDARAB.TTF
2 C:\\WINDOWS\\FONTS\\COURBI.TTF
3 C:\\WINDOWS\\FONTS\\UBUNTU-LIGHT.TTF
4 C:\\WINDOWS\\FONTS\\PALA.TTF
5 C:\\WINDOWS\\FONTS\\COMIC.TTF
:
:
*/
```

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

## Example

```js
const { getPathAll, getProps } = require('node-font-kit');

// Get the path of installed fonts
getPathAll().then((list) => {
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

getProps('path/to/font.otf').then((props) => {
  props.forEach((prop) => console.log(prop));
});

/*
{
  fullname: 'Mplus 1p',
  family: 'Mplus 1p',
  postscriptName: 'Mplus1p-Regular',
  monospace: false,
  weight: 400,
  strech: 1,
  style: 'Normal'
}
*/
```

## API

### `getPathAll() => Promise<string>`

Returns a list of font paths as a Promise.

### `getProps(path: string) => Promise<FontProperty[]>`

Returns a list of font properties as a Promise.

### `FontProperty`

```typescript
type FontProperty = {
  fullname: string;
  family: string;
  postscriptName: string;
  monospace: boolean;
  weight: number;
  strech: number;
  style: 'Normal' | 'Italic' | 'Oblique';
};
```

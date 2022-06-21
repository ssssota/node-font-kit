import { readFileSync } from 'fs';

import { expect, test } from 'vitest';

import { Font } from '../index';

test('sync function from native code', () => {
  const data = readFileSync('__test__/ROBOTO-BLACK.TTF');
  const font = Font.fromBytes(data, 0);
  expect(font.familyName()).toEqual('Roboto');
  expect(font.fullName()).toEqual(
    process.platform === 'win32' ? 'Roboto Black' : 'Roboto',
  );
  expect(font.postscriptName()).toEqual('Roboto-Black');
  expect(font.isMonospace()).toBeFalsy();
  const properties = font.properties();
  expect(properties.style).toEqual('normal');
  expect(properties.weight).toEqual(900);
  expect(properties.stretch).toEqual(1);
});

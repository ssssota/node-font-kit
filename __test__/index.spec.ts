import { join } from 'path';

import { expect, test } from 'vitest';

import { Font } from '../index';

test('sync function from native code', () => {
  const font = Font.fromPath(join(__dirname, 'ROBOTO-BLACK.TTF'), 0);
  expect(font.familyName()).toEqual('Roboto');
  expect(font.fullName()).toEqual('Roboto Black');
  expect(font.postscriptName()).toEqual('Roboto-Black');
  expect(font.isMonospace()).toBeFalsy();
  const properties = font.properties();
  expect(properties.style).toEqual('normal');
  expect(properties.weight).toEqual(900);
  expect(properties.stretch).toEqual(1);
});

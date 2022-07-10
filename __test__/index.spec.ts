import { readFileSync } from 'fs';
import { platform } from 'process';

import { expect, test } from 'vitest';

import { Font } from '../index';

test('sync function from native code', () => {
  const data = readFileSync('__test__/ROBOTO-BLACK.TTF');
  const font = Font.fromBytes(data, 0);
  expect(font.familyName()).toBe('Roboto');
  expect(font.fullName()).toBe(
    platform === 'win32' || platform === 'darwin' ? 'Roboto Black' : 'Roboto',
  );
  expect(font.postscriptName()).toBe('Roboto-Black');
  expect(font.isMonospace()).toBeFalsy();
  const properties = font.properties();
  expect(properties.style).toBe('normal');
  expect(properties.weight).toBe(900);
  expect(properties.stretch).toBe(1);
});

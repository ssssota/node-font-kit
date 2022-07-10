import { readFileSync } from 'fs';
import { join } from 'path';

import { describe, expect, it } from 'vitest';

import { FileType, Font } from '../index';

import { ActualFontData, robotoBlack } from './data';

describe('Font', () => {
  const test = (actual: ActualFontData, fileType?: FileType, font?: Font) => {
    fileType ??= Font.analyzePath(actual.path);
    expect(fileType.isSingle()).toBe(actual.fileType.single);
    expect(fileType.count()).toBe(actual.fileType.count);
    font ??= Font.fromPath(actual.path, 0);
    expect(font.familyName()).toBe(actual.familyName);
    expect(font.fullName()).toBe(actual.fullName);
    expect(font.postscriptName()).toBe(actual.postscriptName);
    expect(font.isMonospace()).toBe(actual.monospace);
    expect(font.properties()).toEqual(actual.properties);
  };

  it('Font(from Path)', () => {
    test(robotoBlack);
  });

  it('Font(from Bytes)', () => {
    const path = join(__dirname, 'ROBOTO-BLACK.TTF');
    const data = readFileSync(path);
    const fileType = Font.analyzeBytes(data);
    const font = Font.fromBytes(data, 0);
    test(robotoBlack, fileType, font);
  });
});

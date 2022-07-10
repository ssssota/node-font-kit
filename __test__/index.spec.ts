import { readFileSync } from 'fs';
import { join } from 'path';
import { fileURLToPath } from 'url';

import { describe, expect, it } from 'vitest';

import { FileType, Font } from '../index';

import { ExpectedFontData, robotoBlack } from './data';

describe('Font', () => {
  const test = (
    exptected: ExpectedFontData,
    actualFileType?: FileType,
    actualFont?: Font,
  ) => {
    const fileType = actualFileType ?? Font.analyzePath(exptected.path);
    expect(fileType.isSingle()).toBe(exptected.fileType.single);
    expect(fileType.count()).toBe(exptected.fileType.count);
    const font = actualFont ?? Font.fromPath(exptected.path, 0);
    expect(font.familyName()).toBe(exptected.familyName);
    expect(font.fullName()).toBe(exptected.fullName);
    expect(font.postscriptName()).toBe(exptected.postscriptName);
    expect(font.isMonospace()).toBe(exptected.monospace);
    expect(font.properties()).toEqual(exptected.properties);
  };

  it('Font(from Path)', () => {
    test(robotoBlack);
  });

  it('Font(from Bytes)', () => {
    const path = join(fileURLToPath(import.meta.url), '../ROBOTO-BLACK.TTF');
    const data = readFileSync(path);
    const fileType = Font.analyzeBytes(data);
    const font = Font.fromBytes(data, 0);
    test(robotoBlack, fileType, font);
  });
});

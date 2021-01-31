import { existsSync } from 'fs';
import { isAbsolute } from 'path';
import { getPathAll, getProps } from '../lib/index';

describe('getPathAll', () => {
  it('should return string array', (done) => {
    getPathAll()
      .then((list) => list.forEach((path) => expect(typeof path).toBe('string')))
      .then(() => done())
      .catch((e) => done(e));
  });

  it('should return exists filepath', (done) => {
    getPathAll()
      .then((list) => list.forEach((path) => expect(existsSync(path)).toBe(true)))
      .then(() => done())
      .catch((e) => done(e));
  });

  it('should return absolute path', (done) => {
    getPathAll()
      .then((list) => list.forEach((path) => expect(isAbsolute(path)).toBe(true)))
      .then(() => done())
      .catch((e) => done(e));
  });

  it('should return unique paths', (done) => {
    getPathAll()
      .then((list) => list.sort().reduce((prev, cur) => {
        expect(cur).not.toBe(prev);
        return cur;
      }, ''))
      .then(() => done())
      .catch((e) => done(e));
  });
});

describe('getProps', () => {
  it('should return expected props (MPLUS1p-Regulat.ttf)', (done) => {
    getProps('./tests/resouces/MPLUS1p-Regular.ttf')
      .then((props) => {
        expect(props.length).toBe(1);
        expect(props[0]?.fullname).toBe('Mplus 1p');
        expect(props[0]?.family).toBe('Mplus 1p');
        expect(props[0]?.postscriptName).toBe('Mplus1p-Regular');
        expect(props[0]?.monospace).toBe(false);
        expect(props[0]?.weight).toBe(400);
        expect(props[0]?.stretch).toBe(1);
        expect(props[0]?.style).toBe('Normal');
      })
      .then(() => done())
      .catch((e) => done(e));
  });
  it('should return expected props (Ubuntu-BoldItalic.ttf)', (done) => {
    getProps('./tests/resouces/Ubuntu-BoldItalic.ttf')
      .then((props) => {
        expect(props.length).toBe(1);
        expect(props[0]?.fullname).toBe('Ubuntu Bold Italic');
        expect(props[0]?.family).toBe('Ubuntu');
        expect(props[0]?.postscriptName).toBe('Ubuntu-BoldItalic');
        expect(props[0]?.monospace).toBe(false);
        expect(props[0]?.weight).toBe(700);
        expect(props[0]?.stretch).toBe(1);
        expect(props[0]?.style).toBe('Italic');
      })
      .then(() => done())
      .catch((e) => done(e));
  });
  it('should return expected props (NotoSansJP-Thin.otf)', (done) => {
    getProps('./tests/resouces/NotoSansJP-Thin.otf')
      .then((props) => {
        expect(props.length).toBe(1);
        expect(props[0]?.fullname).toBe('Noto Sans JP Thin');
        expect(props[0]?.family).toBe('Noto Sans JP');
        expect(props[0]?.postscriptName).toBe('NotoSansJP-Thin');
        expect(props[0]?.monospace).toBe(false);
        expect(props[0]?.weight).toBe(250);
        expect(props[0]?.stretch).toBe(1);
        expect(props[0]?.style).toBe('Normal');
      })
      .then(() => done())
      .catch((e) => done(e));
  });
  it('should return expected props (RobotoMono-Italic-VariableFont_wght.ttf)', (done) => {
    getProps('./tests/resouces/RobotoMono-Italic-VariableFont_wght.ttf')
      .then((props) => {
        expect(props.length).toBe(1);
        // expect(props[0]?.fullname).toBe('Roboto Mono Thin Italic');
        expect(props[0]?.family).toBe('Roboto Mono');
        // expect(props[0]?.postscriptName).toBe('Roboto-Mono-Thin-Italic');
        expect(props[0]?.monospace).toBe(true);
        // expect(props[0]?.weight).toBe(100);
        // expect(props[0]?.stretch).toBe(1);
        // expect(props[0]?.style).toBe('Italic');
      })
      .then(() => done())
      .catch((e) => done(e));
  });
});

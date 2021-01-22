import { existsSync } from 'fs';
import { isAbsolute } from 'path';
import { getPathAll } from '../lib/index';

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

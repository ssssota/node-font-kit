import { assert } from 'console';
import { existsSync, realpathSync } from 'fs';
import { getPathAll } from '../lib/index';

describe('getPathAll', () => {
  it('should return string array', (done) => {
    getPathAll()
      .then((list) => list.forEach((path) => expect(typeof path).toBe('string')))
      .then(done);
  });

  it('should return exists filepath', (done) => {
    getPathAll()
      .then((list) => list.forEach((path) => expect(existsSync(path)).toBe(true)))
      .then(done);
  });

  it('should return absolute path', (done) => {
    getPathAll()
      .then((list) => list.forEach((path) => expect(path).toBe(realpathSync(path))))
      .then(done);
  });
});

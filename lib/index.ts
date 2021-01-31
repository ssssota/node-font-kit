import { FontProperties } from './types';

const addon = require('./index.node');

export const getPathAll = (): Promise<string[]> => new Promise((resolve, reject) => {
  try {
    resolve(addon.getPathAll());
  } catch (e) {
    reject(e);
  }
});

type getPropsResult = (FontProperties | undefined)[];
export const getProps = (
  path: string,
): Promise<getPropsResult> => new Promise((resolve, reject) => {
  try {
    resolve(addon.getProps(path));
  } catch (e) {
    reject(e);
  }
});

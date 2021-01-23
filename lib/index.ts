const addon = require('./index.node');

export const getPathAll = (): Promise<string[]> => new Promise((resolve, reject) => {
  try {
    resolve(addon.getPathAll());
  } catch (e) {
    reject(e);
  }
});

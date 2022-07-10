import { join } from 'path';
import { platform } from 'process';

import type { Properties } from '..';

const runsOnWinOrMac = platform === 'win32' || platform === 'darwin';

export type ExpectedFontData = {
  path: string;
  fileType: { single: boolean; count: number };
  familyName: string;
  fullName: string;
  postscriptName: string;
  monospace: boolean;
  properties: Properties;
};

export const robotoBlack: ExpectedFontData = {
  path: join(__dirname, 'ROBOTO-BLACK.TTF'),
  fileType: {
    single: true,
    count: 1,
  },
  familyName: 'Roboto',
  fullName: runsOnWinOrMac ? 'Roboto Black' : 'Roboto',
  postscriptName: 'Roboto-Black',
  monospace: false,
  properties: {
    style: 'normal',
    weight: 900,
    stretch: 1,
  },
};

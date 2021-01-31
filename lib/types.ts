export type Style = 'Normal' | 'Italic' | 'Oblique';
export type FontProperties = {
  fullname: string;
  family: string;
  postscriptName: string;
  monospace: boolean;
  weight: number;
  stretch: number;
  style: Style;
};

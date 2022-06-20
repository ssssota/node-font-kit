/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Properties {
  style: 'normal' | 'italic' | 'oblique'
  weight: number
  stretch: number
}
export type JsFamilyHandle = FamilyHandle
export class FamilyHandle {
  constructor()
}
export type JsFont = Font
export class Font {
  static fromPath(path: string, fontIndex: number): JsFont
  static fromBytes(fontData: Buffer, fontIndex: number): JsFont
}
export type JsHandle = Handle
export class Handle {
  static fromPath(path: string, fontIndex: number): JsHandle
  static fromMemory(bytes: Buffer, fontIndex: number): JsHandle
  load(): Font
  get path(): string | null
  get fontIndex(): number
}
export type JsSource = Source
export class Source {
  constructor()
  allFonts(): Array<Handle>
  allFamilies(): Array<string>
  selectByPostscriptName(postscriptName: string): Handle
  selectBestMatch(familyNames: Array<string>, properties: Properties): Handle
}

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/**
 * Properties that specify which font in a family to use: e.g. style, weight, and stretchiness.
 *
 * ref. [Properties](https://docs.rs/font-kit/latest/font_kit/properties/struct.Properties.html)
 */
export interface Properties {
  style: 'normal' | 'italic' | 'oblique'
  weight: number
  stretch: number
}
export type JsFamilyHandle = FamilyHandle
/**
 * Encapsulates the information needed to locate and open the fonts in a family.
 *
 * ref. [FamilyHandle](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html)
 */
export class FamilyHandle {
  /**
   * Creates an empty set of family handles.
   *
   * ref. [new](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html#method.new)
   */
  constructor()
  /**
   * Returns true if and only if this set has no fonts in it.
   *
   * ref. [is_empty](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html#method.is_empty)
   */
  isEmpty(): boolean
  /**
   * Returns all the handles in this set.
   *
   * ref. [fonts](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html#method.fonts)
   */
  fonts(): Array<JsHandle>
}
export type JsFileType = FileType
/**
 * The type of a font file: either a single font or a TrueType/OpenType collection.
 *
 * ref. [FileType](https://docs.rs/font-kit/latest/font_kit/file_type/enum.FileType.html)
 */
export class FileType {
  /**
   * The font file represents a single font (.ttf, .otf, .woff, etc.)
   *
   * ref. [Single](https://docs.rs/font-kit/latest/font_kit/file_type/enum.FileType.html#variant.Single)
   */
  static single(): JsFileType
  /**
   * The font file represents a collection of fonts (.ttc, .otc, etc.)
   *
   * ref. [Collection](https://docs.rs/font-kit/latest/font_kit/file_type/enum.FileType.html#variant.Collection)
   */
  static collection(count: number): JsFileType
  /** Returns true if file type is single (not collection). */
  get isSingle(): boolean
  /** Returns number of font in the file. */
  get count(): number
}
export type JsFont = Font
/**
 * A font face loaded into memory.
 *
 * ref. [Font](https://docs.rs/font-kit/latest/font_kit/font/index.html)
 */
export class Font {
  /**
   * Loads a font from the path to a .ttf/.otf/etc. file.
   *
   * If the file is a collection (.ttc/.otc/etc.), font_index specifies the index of the font to load from it. If the file represents a single font, pass 0 for font_index.
   *
   * ref. [from_path](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.from_path)
   */
  static fromPath(path: string, fontIndex: number): JsFont
  /**
   * Loads a font from raw font data (the contents of a .ttf/.otf/etc. file).
   *
   * If the data represents a collection (.ttc/.otc/etc.), font_index specifies the index of the font to load from it. If the data represents a single font, pass 0 for font_index.
   *
   * ref. [from_bytes](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.from_bytes)
   */
  static fromBytes(fontData: Uint8Array, fontIndex: number): JsFont
  /**
   * Determines whether a path points to a supported font, and, if so, what type of font it is.
   *
   * ref. [analyze_path](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.analyze_path)
   */
  static analyzePath(path: string): FileType
  /**
   * Determines whether a blob of raw font data represents a supported font, and, if so, what type of font it is.
   *
   * ref. [analyze_bytes](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.analyze_bytes)
   */
  static analyzeBytes(fontData: Uint8Array): FileType
  /**
   * Returns the PostScript name of the font. This should be globally unique.
   *
   * ref. [postscript_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.postscript_name)
   */
  postscriptName(): string | undefined
  /**
   * Returns the full name of the font (also known as “display name” on macOS).
   *
   * ref. [full_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.full_name)
   */
  fullName(): string
  /**
   * Returns the name of the font family.
   *
   * ref. [family_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.family_name)
   */
  familyName(): string
  /**
   * Returns true if and only if the font is monospace (fixed-width).
   *
   * ref. [is_monospace](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.is_monospace)
   */
  isMonospace(): boolean
  /**
   * Returns the values of various font properties, corresponding to those defined in CSS.
   *
   * ref. [properties](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.properties)
   */
  properties(): Properties
  /**
   * Returns the usual glyph ID for a Unicode character.
   *
   * Be careful with this function; typographically correct character-to-glyph mapping must be done using a shaper such as HarfBuzz. This function is only useful for best-effort simple use cases like “what does character X look like on its own”.
   *
   * ref. [glyph_for_char](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.glyph_for_char)
   */
  glyphForChar(character: string): number | undefined
  /**
   * Returns the glyph ID for the specified glyph name.
   *
   * ref. [glyph_by_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.glyph_by_name)
   */
  glyphByName(name: string): number | undefined
  /**
   * Returns the number of glyphs in the font.
   *
   * Glyph IDs range from 0 inclusive to this value exclusive.
   *
   * ref. [glyph_count](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.glyph_count)
   */
  glyphCount(): number
  /**
   * Returns the raw contents of the OpenType table with the given tag.
   *
   * Tags are four-character codes. A list of tags can be found in the [OpenType specification](https://docs.microsoft.com/en-us/typography/opentype/spec/).
   *
   * ref. [load_font_data](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.load_font_table)
   */
  loadFontData(tableTag: number): Uint8Array | undefined
}
export type JsHandle = Handle
/**
 * Encapsulates the information needed to locate and open a font.
 *
 * This is either the path to the font or the raw in-memory font data.
 *
 * To open the font referenced by a handle, use a loader.
 *
 * ref. [Handle](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html)
 */
export class Handle {
  /**
   * Creates a new handle from a path.
   *
   * font_index specifies the index of the font to choose if the path points to a font collection. If the path points to a single font file, pass 0.
   *
   * ref. [from_path](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#method.from_path)
   */
  static fromPath(path: string, fontIndex: number): JsHandle
  /**
   * Creates a new handle from raw TTF/OTF/etc. data in memory.
   *
   * font_index specifies the index of the font to choose if the memory represents a font collection. If the memory represents a single font file, pass 0.
   *
   * ref. [from_memory](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#method.from_memory)
   */
  static fromMemory(bytes: Uint8Array, fontIndex: number): JsHandle
  /**
   * A convenience method to load this handle with the default loader, producing a Font.
   *
   * ref. [load](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#method.load)
   */
  load(): Font
  /**
   * The path to the font.
   *
   * ref. [path](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#variant.Path.field.path)
   */
  get path(): string
  /**
   * The index of the font, if the path refers to a collection.
   *
   * If the path refers to a single font, this value will be 0.
   *
   * ref. [font_index](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#variant.Path.field.font_index)
   */
  get fontIndex(): number
}
export type JsSource = Source
/**
 * A database of installed fonts that can be queried.
 *
 * ref. [Source](https://docs.rs/font-kit/latest/font_kit/source/trait.Source.html)
 */
export class Source {
  /**
   * Initialize system default source.
   *
   * - Linux: [Fontconfig](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html)
   * - Windows: [DirectWrite](https://docs.rs/font-kit/latest/x86_64-pc-windows-msvc/font_kit/sources/directwrite/struct.DirectWriteSource.html)
   * - Mac: [Core Text](https://docs.rs/font-kit/latest/x86_64-apple-darwin/font_kit/sources/core_text/struct.CoreTextSource.html)
   *
   * ref. [SystemSource](https://docs.rs/font-kit/latest/font_kit/source/index.html#:~:text=SystemSource)
   */
  static system(): JsSource
  /**
   * Returns paths of all fonts installed on the system.
   *
   * ref. [all_fonts](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.all_fonts)
   */
  allFonts(): Array<Handle>
  /**
   * Returns the names of all families installed on the system.
   *
   * ref. [all_families](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.all_families)
   */
  allFamilies(): Array<string>
  /**
   * Looks up a font family by name and returns the handles of all the fonts in that family.
   *
   * ref. [select_family_by_name](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.select_family_by_name)
   */
  selectFamilyByName(familyName: string): FamilyHandle
  /**
   * Selects a font by PostScript name, which should be a unique identifier.
   *
   * The default implementation, which is used by the DirectWrite and the filesystem backends, does a brute-force search of installed fonts to find the one that matches.
   *
   * ref. [select_by_postscript_name](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.select_by_postscript_name)
   */
  selectByPostscriptName(postscriptName: string): Handle
  /**
   * Performs font matching according to the CSS Fonts Level 3 specification and returns the handle.
   *
   * ref. [select_best_match](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.select_best_match)
   */
  selectBestMatch(familyNames: Array<string>, properties: Properties): Handle
}

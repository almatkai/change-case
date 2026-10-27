/// <reference types="@raycast/api">

/* 🚧 🚧 🚧
 * This file is auto-generated from the extension's manifest.
 * Do not modify manually. Instead, update the `package.json` file.
 * 🚧 🚧 🚧 */

/* eslint-disable @typescript-eslint/ban-types */

type ExtensionPreferences = {
  /** Behavior - Uncheck this option to first lowercase the text before applying transformations. See README for details. */
  "preserveCase": boolean,
  /** undefined - Check this option to preserve punctuation marks in their original positions when changing case. This option is only available for Lower Case and Upper Case. */
  "preservePunctuation": boolean,
  /** undefined - Usually, you only need to change the case for one text at a time - check this to pop back to root transforming the text. */
  "popToRoot": boolean,
  /** undefined - Check this to hide the HUD after the action. */
  "hideHUD": boolean,
  /** Title and Sentence Case Exceptions - Words or phrases that should not be modified, separated by commas. */
  "exceptions": string,
  /** Preferred Source - Choose a preferred text source. If no text is selected, clipboard is used. */
  "source": "selected" | "clipboard",
  /** Primary Action - Choose whether the primary action should copy or paste the output. */
  "action": "copy" | "paste",
  /** Prefix Characters - Characters to retain at the beginning of the input. Eg, `_` to keep underscores in `__typename` */
  "prefixCharacters": string,
  /** Suffix Characters - Characters to retain at the end of the input. Eg, `_` to keep underscores in `type_` */
  "suffixCharacters": string,
  /** Enabled Transformations - Enable the "camelCase" Transformation */
  "CamelCase": boolean,
  /** undefined - Enable the "Capital Case" Transformation */
  "CapitalCase": boolean,
  /** undefined - Enable the "CONSTANT_CASE" Transformation */
  "ConstantCase": boolean,
  /** undefined - Enable the "dot.case" Transformation */
  "DotCase": boolean,
  /** undefined - Enable the "Header-Case" Transformation */
  "HeaderCase": boolean,
  /** undefined - Enable the "lower case" Transformation */
  "LowerCase": boolean,
  /** undefined - Enable the "lower First" Transformation */
  "LowerFirst": boolean,
  /** undefined - Enable the "no case" Transformation */
  "NoCase": boolean,
  /** undefined - Enable the "kebab-case" Transformation */
  "KebabCase": boolean,
  /** undefined - Enable the "KEBAB-UPPER-CASE" Transformation */
  "KebabUpperCase": boolean,
  /** undefined - Enable the "PascalCase" Transformation */
  "PascalCase": boolean,
  /** undefined - Enable the "Pascal_Snake_Case" Transformation */
  "PascalSnakeCase": boolean,
  /** undefined - Enable the "path/case" Transformation */
  "PathCase": boolean,
  /** undefined - Enable the "rAndOm cAsE" Transformation */
  "RandomCase": boolean,
  /** undefined - Enable the "Sentence case" Transformation */
  "SentenceCase": boolean,
  /** undefined - Enable the "snake_case" Transformation */
  "SnakeCase": boolean,
  /** undefined - Enable the "SpOnGeCaSe" Transformation (alternating case) */
  "AlternatingCase": boolean,
  /** undefined - Enable the "sWAP cASE" Transformation */
  "SwapCase": boolean,
  /** undefined - Enable the "Title Case" Transformation */
  "TitleCase": boolean,
  /** undefined - Enable the "UPPER CASE" Transformation */
  "UpperCase": boolean,
  /** undefined - Enable the "Upper first" Transformation */
  "UpperFirst": boolean
}

/** Preferences accessible in all the extension's commands */
declare type Preferences = ExtensionPreferences

declare namespace Preferences {
  /** Preferences accessible in the `change-case` command */
  export type ChangeCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-camel-case` command */
  export type ConvertCamelCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-capital-case` command */
  export type ConvertCapitalCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-constant-case` command */
  export type ConvertConstantCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-dot-case` command */
  export type ConvertDotCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-header-case` command */
  export type ConvertHeaderCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-lower-case` command */
  export type ConvertLowerCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-lower-first` command */
  export type ConvertLowerFirst = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-no-case` command */
  export type ConvertNoCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-kebab-case` command */
  export type ConvertKebabCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-kebab-upper-case` command */
  export type ConvertKebabUpperCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-pascal-case` command */
  export type ConvertPascalCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-pascal-snake-case` command */
  export type ConvertPascalSnakeCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-path-case` command */
  export type ConvertPathCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-random-case` command */
  export type ConvertRandomCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-sentence-case` command */
  export type ConvertSentenceCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-snake-case` command */
  export type ConvertSnakeCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-alternating-case` command */
  export type ConvertAlternatingCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-swap-case` command */
  export type ConvertSwapCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-title-case` command */
  export type ConvertTitleCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-upper-case` command */
  export type ConvertUpperCase = ExtensionPreferences & {}
  /** Preferences accessible in the `convert-upper-first` command */
  export type ConvertUpperFirst = ExtensionPreferences & {}
}

declare namespace Arguments {
  /** Arguments passed to the `change-case` command */
  export type ChangeCase = {}
  /** Arguments passed to the `convert-camel-case` command */
  export type ConvertCamelCase = {}
  /** Arguments passed to the `convert-capital-case` command */
  export type ConvertCapitalCase = {}
  /** Arguments passed to the `convert-constant-case` command */
  export type ConvertConstantCase = {}
  /** Arguments passed to the `convert-dot-case` command */
  export type ConvertDotCase = {}
  /** Arguments passed to the `convert-header-case` command */
  export type ConvertHeaderCase = {}
  /** Arguments passed to the `convert-lower-case` command */
  export type ConvertLowerCase = {}
  /** Arguments passed to the `convert-lower-first` command */
  export type ConvertLowerFirst = {}
  /** Arguments passed to the `convert-no-case` command */
  export type ConvertNoCase = {}
  /** Arguments passed to the `convert-kebab-case` command */
  export type ConvertKebabCase = {}
  /** Arguments passed to the `convert-kebab-upper-case` command */
  export type ConvertKebabUpperCase = {}
  /** Arguments passed to the `convert-pascal-case` command */
  export type ConvertPascalCase = {}
  /** Arguments passed to the `convert-pascal-snake-case` command */
  export type ConvertPascalSnakeCase = {}
  /** Arguments passed to the `convert-path-case` command */
  export type ConvertPathCase = {}
  /** Arguments passed to the `convert-random-case` command */
  export type ConvertRandomCase = {}
  /** Arguments passed to the `convert-sentence-case` command */
  export type ConvertSentenceCase = {}
  /** Arguments passed to the `convert-snake-case` command */
  export type ConvertSnakeCase = {}
  /** Arguments passed to the `convert-alternating-case` command */
  export type ConvertAlternatingCase = {}
  /** Arguments passed to the `convert-swap-case` command */
  export type ConvertSwapCase = {}
  /** Arguments passed to the `convert-title-case` command */
  export type ConvertTitleCase = {}
  /** Arguments passed to the `convert-upper-case` command */
  export type ConvertUpperCase = {}
  /** Arguments passed to the `convert-upper-first` command */
  export type ConvertUpperFirst = {}
}


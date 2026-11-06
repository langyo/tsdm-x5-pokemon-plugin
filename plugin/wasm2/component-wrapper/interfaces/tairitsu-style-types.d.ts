/** @module Interface tairitsu:style/types **/
/**
 * # Variants
 * 
 * ## `"flex"`
 * 
 * ## `"grid"`
 * 
 * ## `"block"`
 * 
 * ## `"inline"`
 * 
 * ## `"inline-block"`
 * 
 * ## `"inline-flex"`
 * 
 * ## `"inline-grid"`
 * 
 * ## `"inline-block-alt"`
 * 
 * ## `"contents"`
 * 
 * ## `"list-item"`
 * 
 * ## `"table"`
 * 
 * ## `"inline-table"`
 * 
 * ## `"none"`
 */
export type DisplayValue = "flex" | "grid" | "block" | "inline" | "inline-block" | "inline-flex" | "inline-grid" | "inline-block-alt" | "contents" | "list-item" | "table" | "inline-table" | "none";
/**
 * # Variants
 * 
 * ## `"static-pos"`
 * 
 * ## `"relative"`
 * 
 * ## `"absolute"`
 * 
 * ## `"fixed"`
 * 
 * ## `"sticky"`
 */
export type PositionValue = "static-pos" | "relative" | "absolute" | "fixed" | "sticky";
/**
 * # Variants
 * 
 * ## `"row"`
 * 
 * ## `"column"`
 * 
 * ## `"row-reverse"`
 * 
 * ## `"column-reverse"`
 */
export type FlexDirectionValue = "row" | "column" | "row-reverse" | "column-reverse";
/**
 * # Variants
 * 
 * ## `"nowrap"`
 * 
 * ## `"wrap"`
 * 
 * ## `"wrap-reverse"`
 */
export type FlexWrapValue = "nowrap" | "wrap" | "wrap-reverse";
/**
 * # Variants
 * 
 * ## `"center"`
 * 
 * ## `"space-between"`
 * 
 * ## `"space-around"`
 * 
 * ## `"space-evenly"`
 * 
 * ## `"flex-start"`
 * 
 * ## `"flex-end"`
 * 
 * ## `"start"`
 * 
 * ## `"end"`
 */
export type JustifyContentValue = "center" | "space-between" | "space-around" | "space-evenly" | "flex-start" | "flex-end" | "start" | "end";
/**
 * # Variants
 * 
 * ## `"center"`
 * 
 * ## `"stretch"`
 * 
 * ## `"baseline"`
 * 
 * ## `"flex-start"`
 * 
 * ## `"flex-end"`
 */
export type AlignItemsValue = "center" | "stretch" | "baseline" | "flex-start" | "flex-end";
/**
 * # Variants
 * 
 * ## `"left"`
 * 
 * ## `"right"`
 * 
 * ## `"center"`
 * 
 * ## `"justify"`
 * 
 * ## `"start"`
 * 
 * ## `"end"`
 */
export type TextAlignValue = "left" | "right" | "center" | "justify" | "start" | "end";
/**
 * # Variants
 * 
 * ## `"visible"`
 * 
 * ## `"hidden"`
 * 
 * ## `"scroll"`
 * 
 * ## `"auto"`
 * 
 * ## `"clip"`
 */
export type OverflowValue = "visible" | "hidden" | "scroll" | "auto" | "clip";
/**
 * # Variants
 * 
 * ## `"auto"`
 * 
 * ## `"default"`
 * 
 * ## `"pointer"`
 * 
 * ## `"crosshair"`
 * 
 * ## `"move"`
 * 
 * ## `"text"`
 * 
 * ## `"wait"`
 * 
 * ## `"help"`
 * 
 * ## `"progress"`
 * 
 * ## `"not-allowed"`
 * 
 * ## `"grab"`
 */
export type CursorValue = "auto" | "default" | "pointer" | "crosshair" | "move" | "text" | "wait" | "help" | "progress" | "not-allowed" | "grab";
/**
 * # Variants
 * 
 * ## `"px"`
 * 
 * ## `"percent"`
 * 
 * ## `"em"`
 * 
 * ## `"rem"`
 * 
 * ## `"vw"`
 * 
 * ## `"vh"`
 * 
 * ## `"vmin"`
 * 
 * ## `"vmax"`
 */
export type LengthUnit = "px" | "percent" | "em" | "rem" | "vw" | "vh" | "vmin" | "vmax";
/**
 * # Variants
 * 
 * ## `"auto"`
 * 
 * ## `"none"`
 * 
 * ## `"inherit"`
 * 
 * ## `"initial"`
 * 
 * ## `"unset"`
 * 
 * ## `"revert"`
 */
export type CssValue = "auto" | "none" | "inherit" | "initial" | "unset" | "revert";
export interface StyleDeclaration {
  property: string,
  value: string,
}
/**
 * # Variants
 * 
 * ## `"layout"`
 * 
 * ## `"box-model"`
 * 
 * ## `"flexbox"`
 * 
 * ## `"grid"`
 * 
 * ## `"typography"`
 * 
 * ## `"color"`
 * 
 * ## `"visual"`
 * 
 * ## `"transform"`
 * 
 * ## `"transition"`
 * 
 * ## `"animation"`
 * 
 * ## `"interaction"`
 * 
 * ## `"list-style"`
 * 
 * ## `"table"`
 * 
 * ## `"paged"`
 * 
 * ## `"generated-content"`
 * 
 * ## `"flex-grid-gap"`
 * 
 * ## `"miscellaneous"`
 * 
 * ## `"motion-path"`
 * 
 * ## `"scroll-driven"`
 * 
 * ## `"houdini"`
 * 
 * ## `"svg"`
 * 
 * ## `"hardware"`
 */
export type CssCategory = "layout" | "box-model" | "flexbox" | "grid" | "typography" | "color" | "visual" | "transform" | "transition" | "animation" | "interaction" | "list-style" | "table" | "paged" | "generated-content" | "flex-grid-gap" | "miscellaneous" | "motion-path" | "scroll-driven" | "houdini" | "svg" | "hardware";
export type Property = PropertyKnown | PropertyCustom;
export interface PropertyKnown {
  tag: 'known',
  val: string,
}
export interface PropertyCustom {
  tag: 'custom',
  val: string,
}

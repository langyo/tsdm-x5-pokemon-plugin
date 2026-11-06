/** @module Interface tairitsu:vdom/portal-types **/
/**
 * # Variants
 * 
 * ## `"center"`
 * 
 * ## `"top"`
 * 
 * ## `"top-left"`
 * 
 * ## `"top-right"`
 * 
 * ## `"bottom"`
 * 
 * ## `"bottom-left"`
 * 
 * ## `"bottom-right"`
 * 
 * ## `"left"`
 * 
 * ## `"right"`
 */
export type FixedPosition = "center" | "top" | "top-left" | "top-right" | "bottom" | "bottom-left" | "bottom-right" | "left" | "right";
export type PortalPosition = PortalPositionFollowTrigger | PortalPositionFixed | PortalPositionCustom;
export interface PortalPositionFollowTrigger {
  tag: 'follow-trigger',
}
export interface PortalPositionFixed {
  tag: 'fixed',
  val: FixedPosition,
}
export interface PortalPositionCustom {
  tag: 'custom',
  val: [number, number],
}
/**
 * # Variants
 * 
 * ## `"none"`
 * 
 * ## `"transparent"`
 * 
 * ## `"opaque"`
 * 
 * ## `"blur"`
 */
export type PortalMaskMode = "none" | "transparent" | "opaque" | "blur";

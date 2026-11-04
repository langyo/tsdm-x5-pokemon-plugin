/** @module Interface tairitsu:hooks/state-machine **/
export function transition(state: InteractionState, event: InteractionEvent): InteractionState | undefined;
export function isInteractive(state: InteractionState): boolean;
/**
 * # Variants
 * 
 * ## `"idle"`
 * 
 * ## `"hover"`
 * 
 * ## `"active"`
 * 
 * ## `"focused"`
 * 
 * ## `"disabled"`
 */
export type InteractionState = "idle" | "hover" | "active" | "focused" | "disabled";
/**
 * # Variants
 * 
 * ## `"mouse-enter"`
 * 
 * ## `"mouse-leave"`
 * 
 * ## `"mouse-down"`
 * 
 * ## `"mouse-up"`
 * 
 * ## `"focus"`
 * 
 * ## `"blur"`
 * 
 * ## `"disable"`
 * 
 * ## `"enable"`
 */
export type InteractionEvent = "mouse-enter" | "mouse-leave" | "mouse-down" | "mouse-up" | "focus" | "blur" | "disable" | "enable";

/** @module Interface tairitsu:hooks/animation-types **/
export function evaluateEasing(easing: EasingFunction, t: number): number;
export function lerpF32(start: number, end: number, t: number): number;
/**
 * # Variants
 * 
 * ## `"idle"`
 * 
 * ## `"running"`
 * 
 * ## `"paused"`
 * 
 * ## `"finished"`
 */
export type AnimationState = "idle" | "running" | "paused" | "finished";
/**
 * # Variants
 * 
 * ## `"normal"`
 * 
 * ## `"reverse"`
 * 
 * ## `"alternate"`
 * 
 * ## `"alternate-reverse"`
 */
export type AnimationDirection = "normal" | "reverse" | "alternate" | "alternate-reverse";
export type EasingFunction = EasingFunctionLinear | EasingFunctionEase | EasingFunctionEaseIn | EasingFunctionEaseOut | EasingFunctionEaseInOut | EasingFunctionCubicBezier;
export interface EasingFunctionLinear {
  tag: 'linear',
}
export interface EasingFunctionEase {
  tag: 'ease',
}
export interface EasingFunctionEaseIn {
  tag: 'ease-in',
}
export interface EasingFunctionEaseOut {
  tag: 'ease-out',
}
export interface EasingFunctionEaseInOut {
  tag: 'ease-in-out',
}
export interface EasingFunctionCubicBezier {
  tag: 'cubic-bezier',
  val: [number, number, number, number],
}

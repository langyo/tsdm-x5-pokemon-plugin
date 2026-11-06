/** @module Interface tairitsu:hooks/animation-builder **/
export function defaultAnimationConfig(): AnimationConfig;
export type AnimationDirection = import('./tairitsu-hooks-animation-types.js').AnimationDirection;
export type EasingFunction = import('./tairitsu-hooks-animation-types.js').EasingFunction;
export interface AnimationConfig {
  durationMs: bigint,
  delayMs: bigint,
  iterations: number,
  direction: AnimationDirection,
  easing: EasingFunction,
}

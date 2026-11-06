/** @module Interface tairitsu-browser:full/resize-observer-callbacks@0.2.0 **/
export function onResize(callbackId: bigint, entries: Array<ResizeEntry>): void;
export type DomRect = import('./tairitsu-browser-full-types.js').DomRect;
export interface ResizeEntry {
  target: bigint,
  contentRect: DomRect,
}

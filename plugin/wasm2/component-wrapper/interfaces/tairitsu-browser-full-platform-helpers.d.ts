/** @module Interface tairitsu-browser:full/platform-helpers@0.2.0 **/
export function setTimeout(callbackId: bigint, ms: number): number;
export function clearTimeout(id: number): void;
export function setInterval(callbackId: bigint, ms: number): number;
export function clearInterval(id: number): void;
export function requestAnimationFrame(callbackId: bigint): number;
export function cancelAnimationFrame(id: number): void;

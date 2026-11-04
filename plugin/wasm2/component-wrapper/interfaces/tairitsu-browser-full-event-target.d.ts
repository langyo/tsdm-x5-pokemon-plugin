/** @module Interface tairitsu-browser:full/event-target@0.2.0 **/
export function addEventListener(target: bigint, eventType: string, useCapture: boolean): bigint;
export function removeEventListener(target: bigint, listenerId: bigint): void;
export function preventDefault(event: bigint): void;
export function stopPropagation(event: bigint): void;

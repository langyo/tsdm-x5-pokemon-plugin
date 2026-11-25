/** @module Interface tairitsu-browser:full/event-target@0.2.0 **/
/**
 * @throws {string} To return the error of the result, throw it, or an `Error` that has it as its `payload`.
 */
export function addEventListener(target: bigint, eventType: string, useCapture: boolean): bigint;
/**
 * @throws {string} To return the error of the result, throw it, or an `Error` that has it as its `payload`.
 */
export function removeEventListener(target: bigint, listenerId: bigint): void;
export function preventDefault(event: bigint): void;
export function stopPropagation(event: bigint): void;

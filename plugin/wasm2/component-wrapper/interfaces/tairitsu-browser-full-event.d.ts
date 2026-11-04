/** @module Interface tairitsu-browser:full/event@0.2.0 **/
export function getTarget(self: EventHandle): bigint | undefined;
export function getCurrentTarget(self: EventHandle): bigint | undefined;
export type EventHandle = bigint;

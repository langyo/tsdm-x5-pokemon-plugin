/** @module Interface tairitsu-browser:full/web-socket-callbacks@0.2.0 **/
export function onWebSocketOpen(callbackId: bigint): void;
export function onWebSocketMessage(callbackId: bigint, data: string): void;
export function onWebSocketClose(callbackId: bigint, code: number, reason: string): void;
export function onWebSocketError(callbackId: bigint): void;

/** @module Interface tairitsu-browser:full/wheel-event@0.2.0 **/
export function getDeltaX(self: WheelEventHandle): number;
export function getDeltaY(self: WheelEventHandle): number;
export function getDeltaZ(self: WheelEventHandle): number;
export function getDeltaMode(self: WheelEventHandle): number;
export type WheelEventHandle = bigint;

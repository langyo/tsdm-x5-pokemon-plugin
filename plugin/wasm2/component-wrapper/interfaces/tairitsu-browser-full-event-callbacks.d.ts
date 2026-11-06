/** @module Interface tairitsu-browser:full/event-callbacks@0.2.0 **/
export function onMouseEvent(listenerId: ListenerId, event: EventHandle, data: MouseEventData): void;
export function onKeyboardEvent(listenerId: ListenerId, event: EventHandle, data: KeyboardEventData): void;
export function onFocusEvent(listenerId: ListenerId, event: EventHandle, data: FocusEventData): void;
export function onInputEvent(listenerId: ListenerId, event: EventHandle, data: InputEventData): void;
export function onGenericEvent(listenerId: ListenerId, event: EventHandle, eventType: string): void;
export type ListenerId = bigint;
export type EventHandle = bigint;
export interface MouseEventData {
  clientX: number,
  clientY: number,
  offsetX: number,
  offsetY: number,
  button: number,
  buttons: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
}
export interface KeyboardEventData {
  key: string,
  code: string,
  keyCode: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
  repeat: boolean,
}
export interface FocusEventData {
  relatedTarget?: bigint,
}
export interface InputEventData {
  data?: string,
  inputType: string,
}

/** @module Interface tairitsu:vdom/types **/
export interface DomRect {
  x: number,
  y: number,
  width: number,
  height: number,
}
export interface VText {
  text: string,
}
export interface Style {
  staticStyles: string,
  cssVariables: Array<[string, string]>,
}
export interface Classes {
  staticClasses: string,
}
/**
 * # Variants
 * 
 * ## `"mouse"`
 * 
 * ## `"pen"`
 * 
 * ## `"touch"`
 */
export type PointerType = "mouse" | "pen" | "touch";
export interface MouseEvent {
  clientX: number,
  clientY: number,
  screenX: number,
  screenY: number,
  offsetX: number,
  offsetY: number,
  pageX: number,
  pageY: number,
  movementX: number,
  movementY: number,
  button: number,
  buttons: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
}
export interface KeyboardEvent {
  key: string,
  code: string,
  keyCode: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
  repeat: boolean,
}
export interface FocusEvent {
  relatedTarget?: bigint,
}
export interface InputEvent {
  data?: string,
  inputType?: string,
}
export interface ChangeEvent {
  value: string,
}
export interface WheelEvent {
  deltaX: number,
  deltaY: number,
  deltaZ: number,
  deltaMode: number,
  clientX: number,
  clientY: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
}
export interface TouchPoint {
  identifier: number,
  clientX: number,
  clientY: number,
  screenX: number,
  screenY: number,
  pageX: number,
  pageY: number,
  force?: number,
  radiusX?: number,
  radiusY?: number,
  rotationAngle?: number,
}
export interface TouchEvent {
  touches: Array<TouchPoint>,
  changedTouches: Array<TouchPoint>,
  targetTouches: Array<TouchPoint>,
  timestamp: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
}
export interface PointerEvent {
  pointerId: number,
  pointerType: PointerType,
  isPrimary: boolean,
  width: number,
  height: number,
  pressure: number,
  tiltX: number,
  tiltY: number,
  twist: number,
  clientX: number,
  clientY: number,
  screenX: number,
  screenY: number,
  pageX: number,
  pageY: number,
  button: number,
  buttons: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
}
export interface DragEvent {
  dataTransfer?: string,
  dropEffect?: string,
  effectAllowed?: string,
  clientX: number,
  clientY: number,
  ctrlKey: boolean,
  shiftKey: boolean,
  altKey: boolean,
  metaKey: boolean,
}
export interface TransitionEvent {
  propertyName: string,
  elapsedTime: number,
  pseudoElement: string,
}
export interface AnimationEvent {
  animationName: string,
  pseudoElement: string,
  elapsedTime: number,
  iteration: number,
}
export interface GenericEvent {
  eventType: string,
  target?: bigint,
  currentTarget?: bigint,
  trust: boolean,
}
export type Key = KeyCharacter | KeyBackspace | KeyTab | KeyEnter | KeyShift | KeyControl | KeyAlt | KeyPause | KeyCapsLock | KeyEscape | KeySpace | KeyPageUp | KeyPageDown | KeyEnd | KeyHome | KeyArrowLeft | KeyArrowUp | KeyArrowRight | KeyArrowDown | KeyInsert | KeyDelete | KeyMeta | KeyF1 | KeyF2 | KeyF3 | KeyF4 | KeyF5 | KeyF6 | KeyF7 | KeyF8 | KeyF9 | KeyF10 | KeyF11 | KeyF12 | KeyOther;
export interface KeyCharacter {
  tag: 'character',
  val: string,
}
export interface KeyBackspace {
  tag: 'backspace',
}
export interface KeyTab {
  tag: 'tab',
}
export interface KeyEnter {
  tag: 'enter',
}
export interface KeyShift {
  tag: 'shift',
}
export interface KeyControl {
  tag: 'control',
}
export interface KeyAlt {
  tag: 'alt',
}
export interface KeyPause {
  tag: 'pause',
}
export interface KeyCapsLock {
  tag: 'caps-lock',
}
export interface KeyEscape {
  tag: 'escape',
}
export interface KeySpace {
  tag: 'space',
}
export interface KeyPageUp {
  tag: 'page-up',
}
export interface KeyPageDown {
  tag: 'page-down',
}
export interface KeyEnd {
  tag: 'end',
}
export interface KeyHome {
  tag: 'home',
}
export interface KeyArrowLeft {
  tag: 'arrow-left',
}
export interface KeyArrowUp {
  tag: 'arrow-up',
}
export interface KeyArrowRight {
  tag: 'arrow-right',
}
export interface KeyArrowDown {
  tag: 'arrow-down',
}
export interface KeyInsert {
  tag: 'insert',
}
export interface KeyDelete {
  tag: 'delete',
}
export interface KeyMeta {
  tag: 'meta',
}
export interface KeyF1 {
  tag: 'f1',
}
export interface KeyF2 {
  tag: 'f2',
}
export interface KeyF3 {
  tag: 'f3',
}
export interface KeyF4 {
  tag: 'f4',
}
export interface KeyF5 {
  tag: 'f5',
}
export interface KeyF6 {
  tag: 'f6',
}
export interface KeyF7 {
  tag: 'f7',
}
export interface KeyF8 {
  tag: 'f8',
}
export interface KeyF9 {
  tag: 'f9',
}
export interface KeyF10 {
  tag: 'f10',
}
export interface KeyF11 {
  tag: 'f11',
}
export interface KeyF12 {
  tag: 'f12',
}
export interface KeyOther {
  tag: 'other',
  val: string,
}
export interface FileData {
  name: string,
  mimeType: string,
  size: bigint,
}
export interface FormData {
  values: Array<[string, string]>,
  files: Array<FileData>,
}

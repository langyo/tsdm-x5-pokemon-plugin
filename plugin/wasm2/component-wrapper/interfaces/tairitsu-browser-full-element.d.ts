/** @module Interface tairitsu-browser:full/element@0.2.0 **/
export function getBoundingClientRect(self: ElementHandle): DomRect;
export function getScrollTop(self: ElementHandle): number;
export function setScrollTop(self: ElementHandle, value: number): void;
export function getScrollHeight(self: ElementHandle): number;
export function getClientHeight(self: ElementHandle): number;
export function getClassList(self: ElementHandle): bigint;
export function getAttribute(self: ElementHandle, qualifiedName: string): string | undefined;
export function setAttribute(self: ElementHandle, qualifiedName: string, value: string): void;
export function removeAttribute(self: ElementHandle, qualifiedName: string): void;
export function setInnerHtml(self: ElementHandle, value: string): void;
export type ElementHandle = bigint;
export type DomRect = import('./tairitsu-browser-full-types.js').DomRect;

/** @module Interface tairitsu-browser:full/html-input-element@0.2.0 **/
export function setChecked(self: HtmlInputElementHandle, value: boolean): void;
export function getType(self: HtmlInputElementHandle): string;
export function getValue(self: HtmlInputElementHandle): string;
export function setValue(self: HtmlInputElementHandle, value: string): void;
export type HtmlInputElementHandle = bigint;

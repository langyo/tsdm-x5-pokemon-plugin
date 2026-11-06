/** @module Interface tairitsu-browser:full/css-style-declaration@0.2.0 **/
export function getPropertyValue(self: CssStyleDeclarationHandle, property: string): string;
export function setProperty(self: CssStyleDeclarationHandle, property: string, value: string, priority: string | undefined): void;
export type CssStyleDeclarationHandle = bigint;

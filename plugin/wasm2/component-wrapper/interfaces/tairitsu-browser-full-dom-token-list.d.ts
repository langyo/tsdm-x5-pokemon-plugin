/** @module Interface tairitsu-browser:full/dom-token-list@0.2.0 **/
export function contains(self: DomTokenListHandle, token: string): boolean;
export function add(self: DomTokenListHandle, tokens: Array<string>): void;
export function remove(self: DomTokenListHandle, tokens: Array<string>): void;
export type DomTokenListHandle = bigint;

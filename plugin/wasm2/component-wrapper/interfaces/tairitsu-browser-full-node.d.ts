/** @module Interface tairitsu-browser:full/node@0.2.0 **/
export function getChildNodes(self: NodeHandle): bigint;
export function getFirstChild(self: NodeHandle): bigint | undefined;
export function setTextContent(self: NodeHandle, value: string | undefined): void;
export function insertBefore(self: NodeHandle, node: bigint, child: bigint | undefined): bigint;
export function appendChild(self: NodeHandle, node: bigint): bigint;
export function removeChild(self: NodeHandle, child: bigint): bigint;
export type NodeHandle = bigint;

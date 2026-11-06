/** @module Interface tairitsu-browser:full/mutation-observer-callbacks@0.2.0 **/
export function onMutate(callbackId: bigint, entries: Array<MutationEntry>): void;
export interface MutationEntry {
  mutationType: string,
  target: bigint,
  addedNodes?: bigint,
  removedNodes?: bigint,
  previousSibling?: bigint,
  nextSibling?: bigint,
  attributeName?: string,
  attributeNamespace?: string,
  oldValue?: string,
}

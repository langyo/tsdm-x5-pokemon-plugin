// node-list 接口：tarball glue 缺失，按 0.5.26 world 手写。
// 句柄表必须用 glue-core 的全局 __nodeListHandles（node.getChildNodes
// 经 __storeNodeList 存的就是这张表），跨模块不能各用各的本地表。
export function getLength(self) {
  const list = globalThis.__nodeListHandles && globalThis.__nodeListHandles.get(self);
  return list ? list.length : 0;
}
export function item(self, index) {
  const list = globalThis.__nodeListHandles && globalThis.__nodeListHandles.get(self);
  if (!list || index >= list.length) return undefined;
  const node = list[index];
  if (!node) return undefined;
  return globalThis.__storeNode(node);
}

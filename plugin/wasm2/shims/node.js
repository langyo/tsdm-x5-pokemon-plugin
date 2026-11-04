// shim for tairitsu-browser:full/node
import { INTERFACES } from '../glue/dom/index.js';
const obj = INTERFACES['@tairitsu-glue/node'];
export const appendChild = obj.appendChild;
export const getFirstChild = obj.getFirstChild;
export const insertBefore = obj.insertBefore;
export const removeChild = obj.removeChild;
export const setTextContent = obj.setTextContent;
// getChildNodes：0.5.26 world 新增（tarball glue 未实现）。
// 关键：返回 Element-only 集合（node.children 而非 childNodes）——
// diff 的子索引只数元素节点；走 childNodes 会因空白文本节点错位，
// UpdateChild/InsertChild 会打到错误的 DOM 节点（静默失败）。
export function getChildNodes(self) {
  const node = globalThis.__lookupNode(self);
  const list = node.children;
  return globalThis.__storeNodeList(list);
}

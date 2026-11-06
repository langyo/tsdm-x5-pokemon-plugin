// shim for tairitsu-browser:full/node
import { INTERFACES } from '../glue/dom/index.js?v=1793948672';
const obj = INTERFACES['@tairitsu-glue/node'];
export const appendChild = obj.appendChild;
export const getFirstChild = obj.getFirstChild;
export const getParentElement = obj.getParentElement;
export const insertBefore = obj.insertBefore;
export const removeChild = obj.removeChild;
export const setTextContent = obj.setTextContent;
// getChildNodes：0.5.26 world 新增（tarball glue 未实现）。
// 必须返回 childNodes（全节点）：diff 的子索引把 Text vnode 也计入
// （如 span 的动态文本是 child 0），用 element-only 的 children 会
// 让 UpdateText/UpdateChild 寻址错位（静默失败）。
export function getChildNodes(self) {
  const node = globalThis.__lookupNode(self);
  const list = node.childNodes;
  return globalThis.__storeNodeList(list);
}

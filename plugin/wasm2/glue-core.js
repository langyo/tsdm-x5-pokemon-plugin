(function(){
'use strict';
if (!globalThis.__cssStyleDeclarationHandles) globalThis.__cssStyleDeclarationHandles = new Map();
if (!globalThis.__dispatchingEvents) globalThis.__dispatchingEvents = new Set();
if (!globalThis.__documentHandles) globalThis.__documentHandles = new Map();
if (!globalThis.__domRectHandles) globalThis.__domRectHandles = new Map();
if (!globalThis.__elementHandles) globalThis.__elementHandles = new Map();
if (!globalThis.__eventHandles) globalThis.__eventHandles = new Map();
if (!globalThis.__listenerHandles) globalThis.__listenerHandles = new (class extends Map {  set(h, info) {    for (const [k, v] of this) {      if (k !== h && v.element === info.element && v.type === info.type) {        try { v.element.removeEventListener(v.type, v.listener); } catch (e) {}        this.delete(k);      }    }    return super.set(h, info);  }})();
if (!globalThis.__mutationRecordHandles) globalThis.__mutationRecordHandles = new Map();
if (!globalThis.__nodeListHandles) globalThis.__nodeListHandles = new Map();
if (!globalThis.__resizeObserverEntryHandles) globalThis.__resizeObserverEntryHandles = new Map();
if (!globalThis.__resizeObserverSizeHandles) globalThis.__resizeObserverSizeHandles = new Map();
if (!globalThis.__nextCssStyleDeclarationHandle) globalThis.__nextCssStyleDeclarationHandle = 1n;
if (!globalThis.__nextDomRectHandle) globalThis.__nextDomRectHandle = 1n;
if (!globalThis.__nextEventHandle) globalThis.__nextEventHandle = 1n;
if (!globalThis.__nextHandle) globalThis.__nextHandle = 1n;
if (!globalThis.__nextListenerHandle) globalThis.__nextListenerHandle = 1n;
if (!globalThis.__nextMutationRecord) globalThis.__nextMutationRecord = 1n;
if (!globalThis.__nextNodeList) globalThis.__nextNodeList = 1n;
if (!globalThis.__nextResizeObserverEntry) globalThis.__nextResizeObserverEntry = 1n;
if (!globalThis.__nextResizeObserverSizeHandle) globalThis.__nextResizeObserverSizeHandle = 1n;

// 统一句柄空间：元素/文本/文档都是节点，全部落在 __allNodes，
// 再按类别写入分表（分表供域模块直接 .get() 的场景）。
// handle 分配集中于 __nextHandle，保证跨表唯一。
//
// 句柄按对象幂等（WeakMap 反查）：同一个 DOM 节点每次被解析（如
// apply 期间反复 get_child_at → nodeList.item）必须返回同一个句柄。
// 否则 wasm 侧的「元素身份」每次都变，按 (句柄, 事件) 键的监听器
// 去重全部失配——重渲染一次叠一个 DOM 监听器，第 N 次点击触发
// N 次 handler（实测一个按钮上堆到 7 个 click 监听）。
if (!globalThis.__allNodes) globalThis.__allNodes = new Map();
if (!globalThis.__nodeIdentity) globalThis.__nodeIdentity = new WeakMap();
const storeDom = function (obj, kindTable) {
  let h = globalThis.__nodeIdentity.get(obj);
  if (h !== undefined) {
    if (kindTable) kindTable.set(h, obj);
    return h;
  }
  h = globalThis.__nextHandle; globalThis.__nextHandle = h + 1n;
  globalThis.__allNodes.set(h, obj);
  globalThis.__nodeIdentity.set(obj, h);
  if (kindTable) kindTable.set(h, obj);
  return h;
};
if (!globalThis.__storeElement) globalThis.__storeElement = function (obj) {
  return storeDom(obj, globalThis.__elementHandles);
};
if (!globalThis.__storeText) globalThis.__storeText = function (obj) {
  return storeDom(obj, null);
};
if (!globalThis.__storeNode) globalThis.__storeNode = function (obj) {
  return storeDom(obj, null);
};
if (!globalThis.__lookupNode) globalThis.__lookupNode = function (h) {
  const obj = globalThis.__allNodes.get(h);
  if (obj === undefined) throw new Error('node handle ' + h + ' not found');
  return obj;
};
if (!globalThis.__lookupElement) globalThis.__lookupElement = function (h) {
  const obj = globalThis.__elementHandles.get(h) ?? globalThis.__allNodes.get(h);
  if (obj === undefined) throw new Error('element handle ' + h + ' not found');
  return obj;
};
if (!globalThis.__lookupText) globalThis.__lookupText = function (h) {
  const obj = globalThis.__allNodes.get(h);
  if (obj === undefined) throw new Error('text handle ' + h + ' not found');
  return obj;
};
if (!globalThis.__storeNodeList) globalThis.__storeNodeList = function (obj) {
  const h = globalThis.__nextNodeList; globalThis.__nextNodeList = h + 1n;
  globalThis.__nodeListHandles.set(h, obj); return h;
};
if (!globalThis.__setWasmExports) globalThis.__setWasmExports = function (exports) {
  globalThis.__wasmExports = exports;
};
if (!globalThis.__TAIRITSU_GLUE__) globalThis.__TAIRITSU_GLUE__ = { INTERFACES: {} };

})();
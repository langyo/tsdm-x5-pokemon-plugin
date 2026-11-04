(function(){
'use strict';
if (!globalThis.__cssStyleDeclarationHandles) globalThis.__cssStyleDeclarationHandles = new Map();
if (!globalThis.__dispatchingEvents) globalThis.__dispatchingEvents = new Set();
if (!globalThis.__documentHandles) globalThis.__documentHandles = new Map();
if (!globalThis.__domRectHandles) globalThis.__domRectHandles = new Map();
if (!globalThis.__elementHandles) globalThis.__elementHandles = new Map();
if (!globalThis.__eventHandles) globalThis.__eventHandles = new Map();
if (!globalThis.__listenerHandles) globalThis.__listenerHandles = new Map();
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
if (!globalThis.__allNodes) globalThis.__allNodes = new Map();
const storeDom = function (obj, kindTable) {
  const h = globalThis.__nextHandle; globalThis.__nextHandle = h + 1n;
  globalThis.__allNodes.set(h, obj);
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
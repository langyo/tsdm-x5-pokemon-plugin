// shim for tairitsu-browser:full/platform-helpers
import { INTERFACES } from '../glue/platform/index.js';
const obj = INTERFACES['@tairitsu-glue/platform-helpers'];
export const cancelAnimationFrame = obj.cancelAnimationFrame;
export const clearInterval = obj.clearInterval;
export const clearTimeout = obj.clearTimeout;
export const requestAnimationFrame = obj.requestAnimationFrame;
export const setInterval = obj.setInterval;
export const setTimeout = obj.setTimeout;
// fetchPromise：tarball glue 传原始 Response（WIT 签名期望 String），修正为 text()。
export function fetchPromise(url, optionsJson) {
  const id = globalThis.__nextHandle ?? 1n;
  globalThis.__nextHandle = id + 1n;
  let options;
  try { options = optionsJson ? JSON.parse(optionsJson) : undefined; } catch { options = undefined; }
  const pcb = () => {
    const exports = globalThis.__wasmExports || {};
    return exports["tairitsu-browser:full/promise-callbacks@0.2.0"] || {};
  };
  fetch(url, options).then(async (resp) => {
    const text = await resp.text();
    const iface = pcb();
    (iface.onPromiseResolved || iface.on_promise_resolved || (() => {}))(id, text);
  }).catch((err) => {
    const iface = pcb();
    (iface.onPromiseRejected || iface.on_promise_rejected || (() => {}))(id, String(err));
  });
  return id;
}

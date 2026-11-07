// shim for tairitsu-browser:full/platform-helpers
import { INTERFACES } from '../glue/platform/index.js?v=1794033264';
const obj = INTERFACES['@tairitsu-glue/platform-helpers'];
export const cancelAnimationFrame = obj.cancelAnimationFrame;
export const clearInterval = obj.clearInterval;
export const clearTimeout = obj.clearTimeout;
export const copyToClipboard = obj.copyToClipboard;
export const fetchPromise = obj.fetchPromise;
export const readClipboard = obj.readClipboard;
export const requestAnimationFrame = obj.requestAnimationFrame;
export const setInterval = obj.setInterval;
export const setTimeout = obj.setTimeout;

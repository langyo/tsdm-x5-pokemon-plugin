// shim for tairitsu-browser:full/resize-observer
import { INTERFACES } from '../glue/resize-observer/index.js?v=1795769315';
const obj = INTERFACES['@tairitsu-glue/resize-observer'];
export const observe = obj.observe;
export const unobserve = obj.unobserve;
export const disconnect = obj.disconnect;

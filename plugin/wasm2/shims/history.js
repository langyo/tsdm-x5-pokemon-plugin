// shim for tairitsu-browser:full/history
import { INTERFACES } from '../glue/html/index.js?v=1795723783';
const obj = INTERFACES['@tairitsu-glue/history'];
export const pushState = obj.pushState;
export const replaceState = obj.replaceState;

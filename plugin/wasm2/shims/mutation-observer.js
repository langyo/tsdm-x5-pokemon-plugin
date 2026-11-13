// shim for tairitsu-browser:full/mutation-observer
import { INTERFACES } from '../glue/observers/index.js?v=1794091609';
const obj = INTERFACES['@tairitsu-glue/mutation-observer'];
export const observe = obj.observe;
export const disconnect = obj.disconnect;

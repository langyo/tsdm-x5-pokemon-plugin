// shim for tairitsu-browser:full/non-element-parent-node
import { INTERFACES } from '../glue/dom/index.js';
const obj = INTERFACES['@tairitsu-glue/non-element-parent-node'];
export const getElementById = obj.getElementById;

// shim for tairitsu-browser:full/parent-node
import { INTERFACES } from '../glue/dom/index.js?v=1794000615';
const obj = INTERFACES['@tairitsu-glue/parent-node'];
export const querySelector = obj.querySelector;

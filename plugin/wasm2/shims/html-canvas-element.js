// shim for tairitsu-browser:full/html-canvas-element
import { INTERFACES } from '../glue/platform/index.js?v=1795720671';
const obj = INTERFACES['@tairitsu-glue/html-canvas-element'];
export const getContext = obj.getContext;

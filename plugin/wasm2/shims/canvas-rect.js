// shim for tairitsu-browser:full/canvas-rect
import { INTERFACES } from '../glue/platform/index.js?v=1795715499';
const obj = INTERFACES['@tairitsu-glue/canvas-rect'];
export const clearRect = obj.clearRect;
export const fillRect = obj.fillRect;

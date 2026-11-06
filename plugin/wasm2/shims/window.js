// shim for tairitsu-browser:full/window
import { INTERFACES } from '../glue/html/index.js?v=1793993531';
const obj = INTERFACES['@tairitsu-glue/window'];
export const getComputedStyle = obj.getComputedStyle;
export const getInnerWidth = obj.getInnerWidth;

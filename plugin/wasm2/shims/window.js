// shim for tairitsu-browser:full/window
import { INTERFACES } from '../glue/html/index.js?v=1794089442';
const obj = INTERFACES['@tairitsu-glue/window'];
export const getComputedStyle = obj.getComputedStyle;
export const getInnerHeight = obj.getInnerHeight;
export const getInnerWidth = obj.getInnerWidth;
export const getScrollY = obj.getScrollY;
export const matchMedia = obj.matchMedia;

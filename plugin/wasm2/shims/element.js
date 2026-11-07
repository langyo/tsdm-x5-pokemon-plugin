// shim for tairitsu-browser:full/element
import { INTERFACES } from '../glue/dom/index.js?v=1794030310';
const obj = INTERFACES['@tairitsu-glue/element'];
export const getAttribute = obj.getAttribute;
export const getBoundingClientRect = obj.getBoundingClientRect;
export const getClassList = obj.getClassList;
export const getClientHeight = obj.getClientHeight;
export const getScrollHeight = obj.getScrollHeight;
export const getScrollTop = obj.getScrollTop;
export const getTagName = obj.getTagName;
export const removeAttribute = obj.removeAttribute;
export const setAttribute = obj.setAttribute;
export const setInnerHtml = obj.setInnerHtml;
export const setScrollTop = obj.setScrollTop;

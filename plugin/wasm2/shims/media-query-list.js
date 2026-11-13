// shim for tairitsu-browser:full/media-query-list
import { INTERFACES } from '../glue/html/index.js?v=1794091609';
const obj = INTERFACES['@tairitsu-glue/media-query-list'];
export const addListener = obj.addListener;
export const getMatches = obj.getMatches;
export const getMedia = obj.getMedia;
export const removeListener = obj.removeListener;

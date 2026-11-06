// shim for tairitsu-browser:full/document
import { INTERFACES } from '../glue/dom/index.js?v=1793999497';
const obj = INTERFACES['@tairitsu-glue/document'];
export const createElement = obj.createElement;
export const createTextNode = obj.createTextNode;
export const getBody = obj.getBody;

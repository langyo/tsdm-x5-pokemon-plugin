// shim for tairitsu-browser:full/document
import { INTERFACES } from '../glue/dom/index.js?v=1794091609';
const obj = INTERFACES['@tairitsu-glue/document'];
export const createElement = obj.createElement;
export const createTextNode = obj.createTextNode;
export const elementFromPoint = obj.elementFromPoint;
export const execCommand = obj.execCommand;
export const getBody = obj.getBody;

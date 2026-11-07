// shim for tairitsu-browser:full/css-style-declaration
import { INTERFACES } from '../glue/css/index.js?v=1794089442';
const obj = INTERFACES['@tairitsu-glue/css-style-declaration'];
export const getPropertyValue = obj.getPropertyValue;
export const setProperty = obj.setProperty;

// shim for tairitsu-browser:full/event-target
import { INTERFACES } from '../glue/events/index.js?v=1794050367';
const obj = INTERFACES['@tairitsu-glue/event-target'];
export const addEventListener = obj.addEventListener;
export const preventDefault = obj.preventDefault;
export const removeEventListener = obj.removeEventListener;
export const stopPropagation = obj.stopPropagation;

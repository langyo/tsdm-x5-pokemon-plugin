// shim for tairitsu-browser:full/event
import { INTERFACES } from '../glue/events/index.js?v=1793948672';
const obj = INTERFACES['@tairitsu-glue/event'];
export const getCurrentTarget = obj.getCurrentTarget;
export const getTarget = obj.getTarget;

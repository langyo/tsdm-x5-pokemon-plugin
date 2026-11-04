// shim for tairitsu-browser:full/wheel-event
import { INTERFACES } from '../glue/events/index.js';
const obj = INTERFACES['@tairitsu-glue/wheel-event'];
export const getDeltaMode = obj.getDeltaMode;
export const getDeltaX = obj.getDeltaX;
export const getDeltaY = obj.getDeltaY;
export const getDeltaZ = obj.getDeltaZ;

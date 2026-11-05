// shim for tairitsu-browser:full/pointer-event
import { INTERFACES } from '../glue/events/index.js?v=1793886110';
const obj = INTERFACES['@tairitsu-glue/pointer-event'];
export const getHeight = obj.getHeight;
export const getIsPrimary = obj.getIsPrimary;
export const getPointerId = obj.getPointerId;
export const getPointerType = obj.getPointerType;
export const getPressure = obj.getPressure;
export const getTangentialPressure = obj.getTangentialPressure;
export const getTiltX = obj.getTiltX;
export const getTiltY = obj.getTiltY;
export const getTwist = obj.getTwist;
export const getWidth = obj.getWidth;

/** @module Interface tairitsu-browser:full/geolocation-callbacks@0.2.0 **/
export function onPositionSuccess(callbackId: bigint, position: GeoPosition): void;
export function onPositionError(callbackId: bigint, error: GeoPositionError): void;
export interface GeoPosition {
  latitude: number,
  longitude: number,
  altitude?: number,
  accuracy: number,
  altitudeAccuracy?: number,
  heading?: number,
  speed?: number,
}
export interface GeoPositionError {
  code: number,
  message: string,
}

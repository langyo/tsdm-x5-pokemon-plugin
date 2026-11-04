/** @module Interface tairitsu:hooks/async-resource **/
export function resourceStatusQuery(resourceId: bigint): ResourceStatus | undefined;
/**
 * # Variants
 * 
 * ## `"loading"`
 * 
 * ## `"ready"`
 * 
 * ## `"error"`
 */
export type ResourceStatus = "loading" | "ready" | "error";

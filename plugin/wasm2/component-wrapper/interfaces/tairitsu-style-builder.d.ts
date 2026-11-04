/** @module Interface tairitsu:style/builder **/
export function buildStyleString(declarations: Array<[string, string]>): string;
export function cssPropertyToString(name: string): string;
export function cssLengthToString(value: number, unit: string): string;

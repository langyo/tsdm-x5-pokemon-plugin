/* tslint:disable */
/* eslint-disable */

export class JSOwner {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class WebHandle {
    free(): void;
    [Symbol.dispose](): void;
    has_panicked(): boolean;
    constructor();
    start(): Promise<void>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_webhandle_free: (a: number, b: number) => void;
    readonly webhandle_has_panicked: (a: number) => number;
    readonly webhandle_new: () => number;
    readonly webhandle_start: (a: number) => number;
    readonly __wbg_jsowner_free: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_5465: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_5645: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_7701: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_7751: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_5648: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_5649: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_7702: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_5466: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_5646: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

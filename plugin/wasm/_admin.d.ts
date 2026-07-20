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
    destroy(): void;
    has_panicked(): boolean;
    constructor();
    panic_callstack(): string | undefined;
    panic_message(): string | undefined;
    start(): Promise<void>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_webhandle_free: (a: number, b: number) => void;
    readonly webhandle_destroy: (a: number) => void;
    readonly webhandle_has_panicked: (a: number) => number;
    readonly webhandle_new: () => number;
    readonly webhandle_panic_callstack: (a: number, b: number) => void;
    readonly webhandle_start: (a: number) => number;
    readonly webhandle_panic_message: (a: number, b: number) => void;
    readonly __wbg_jsowner_free: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_12904: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_12918: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_10871: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_10869: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_10365: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_10475: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_10873: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export5: (a: number, b: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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

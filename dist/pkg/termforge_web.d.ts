/* tslint:disable */
/* eslint-disable */

export class WasmAsteroidsGame {
    free(): void;
    [Symbol.dispose](): void;
    key_down(key: string): void;
    key_up(key: string): void;
    last_sound(): string | undefined;
    constructor(width: number, height: number);
    reset(): void;
    /**
     * Advance game state by dt and return the ANSI diff string to be passed into xterm.js
     */
    tick(dt: number): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmasteroidsgame_free: (a: number, b: number) => void;
    readonly wasmasteroidsgame_key_down: (a: number, b: number, c: number) => void;
    readonly wasmasteroidsgame_key_up: (a: number, b: number, c: number) => void;
    readonly wasmasteroidsgame_last_sound: (a: number, b: number) => void;
    readonly wasmasteroidsgame_new: (a: number, b: number) => number;
    readonly wasmasteroidsgame_reset: (a: number) => void;
    readonly wasmasteroidsgame_tick: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
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

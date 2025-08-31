/* tslint:disable */
export interface Scoring {
    match_score: number;
    mismatch_penalty: number;
    gap_open_penalty: number;
    gap_extend_penalty: number;
    prefix_bonus: number;
    offset_prefix_bonus: number;
    capitalization_bonus: number;
    matching_case_bonus: number;
    exact_match_bonus: number;
    delimiters: string;
    delimiter_bonus: number;
}

export interface Config {
    prefilter: boolean;
    max_typos: number | null;
    sort: boolean;
    scoring: Scoring;
}

export interface Match {
    score: number;
    index: number;
    exact: boolean;
}

export interface MatchIndices {
    score: number;
    indices: number[];
    exact: boolean;
}

export interface ComparisonResult {
    needle: string;
    haystack: string;
    needle_index: number;
    haystack_index: number;
    score: number;
    exact: boolean;
}

/* eslint-disable */
export function create_custom_config(prefilter: boolean, max_typos: number | null | undefined, sort: boolean, scoring_js?: Scoring): Config;
export function create_default_config(): Config;
export function create_default_scoring(): Scoring;
export function create_custom_scoring(match_score?: number | null, mismatch_penalty?: number | null, gap_open_penalty?: number | null, gap_extend_penalty?: number | null, prefix_bonus?: number | null, offset_prefix_bonus?: number | null, capitalization_bonus?: number | null, matching_case_bonus?: number | null, exact_match_bonus?: number | null, delimiter_bonus?: number | null, delimiters?: string | null): Scoring;
export class WasmMatcher {
  free(): void;
  matchList(needle: string, haystacks: string[], config_js?: Config): Match[];
  compareAll(items: string[], config_js?: Config, min_score?: number): ComparisonResult[];
  matchIndices(needle: string, haystack: string, config_js?: Config): MatchIndices | null;
  constructor();
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmmatcher_free: (a: number, b: number) => void;
  readonly create_custom_config: (a: number, b: number, c: number, d: any) => [number, number, number];
  readonly create_custom_scoring: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => [number, number, number];
  readonly create_default_config: () => [number, number, number];
  readonly create_default_scoring: () => [number, number, number];
  readonly wasmmatcher_compareAll: (a: number, b: number, c: number, d: any, e: number) => [number, number, number];
  readonly wasmmatcher_matchIndices: (a: number, b: number, c: number, d: number, e: number, f: any) => [number, number, number];
  readonly wasmmatcher_matchList: (a: number, b: number, c: number, d: number, e: number, f: any) => [number, number, number];
  readonly wasmmatcher_new: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __externref_table_alloc: () => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
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

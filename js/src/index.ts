export interface Match {
  /** Match score (higher is better) */
  score: number;
  /** Index in the original haystacks array */
  index: number;
  /** Whether the needle matched the haystack exactly */
  exact: boolean;
}

export interface MatchIndices {
  /** Match score (higher is better) */
  score: number;
  /** Index in the original haystacks array */
  index: number;
  /** Whether the needle matched the haystack exactly */
  exact: boolean;
  /** Character indices in the haystack that matched the needle (in reverse order) */
  indices: number[];
}

export interface Config {
  /** Maximum typos (missing needle chars) allowed before filtering out. Default: 0 */
  maxTypos?: number;
  /** Sort results by score descending. Default: true */
  sort?: boolean;
}

type WasmExports = {
  matchList(needle: string, haystacks: string[], config: Config): Match[];
  matchListIndices(
    needle: string,
    haystacks: string[],
    config: Config
  ): MatchIndices[];
  Matcher: {
    new (needle: string, config: Config): WasmMatcher;
  };
};

type WasmMatcher = {
  matchList(haystacks: string[]): Match[];
  matchListIndices(haystacks: string[]): MatchIndices[];
  setNeedle(needle: string): void;
  free(): void;
};

let wasm: WasmExports | null = null;

const defaultConfig: Config = { maxTypos: 0, sort: true };

/**
 * Initialize the WASM module. Must be called before any other function.
 *
 * @param input - URL to the .wasm file, a fetch Response, or an ArrayBuffer.
 *
 * @example
 * ```ts
 * import { init } from 'frizbee';
 * import wasmUrl from 'frizbee/frizbee.wasm?url';
 * await init(wasmUrl);
 * ```
 */
export async function init(
  input: string | URL | Response | BufferSource | WebAssembly.Module
): Promise<void> {
  // Dynamic import of wasm-bindgen generated glue
  const bindgen = await import("../pkg/frizbee.js");
  await bindgen.default(input);
  wasm = bindgen as unknown as WasmExports;
}

function getWasm(): WasmExports {
  if (!wasm) {
    throw new Error(
      "frizbee: WASM not initialized. Call init(wasmUrl) first."
    );
  }
  return wasm;
}

/**
 * Match a needle against a list of haystacks.
 * Returns matches sorted by score (descending) by default.
 */
export function matchList(
  needle: string,
  haystacks: string[],
  config?: Config
): Match[] {
  return getWasm().matchList(
    needle,
    haystacks,
    config ?? defaultConfig
  ) as Match[];
}

/**
 * Match a needle against a list of haystacks, including matched character indices.
 * Returns matches sorted by score (descending) by default.
 */
export function matchListIndices(
  needle: string,
  haystacks: string[],
  config?: Config
): MatchIndices[] {
  return getWasm().matchListIndices(
    needle,
    haystacks,
    config ?? defaultConfig
  ) as MatchIndices[];
}

/**
 * Reusable matcher for matching one needle against many haystack lists.
 * More efficient than `matchList` when the needle doesn't change between calls.
 *
 * @example
 * ```ts
 * const matcher = createMatcher('foo', { maxTypos: 1 });
 * const results1 = matcher.matchList(['fooBar', 'barBaz']);
 * const results2 = matcher.matchList(['hello', 'fooWorld']);
 * matcher.free(); // release WASM memory when done
 * ```
 */
export function createMatcher(
  needle: string,
  config?: Config
): {
  matchList(haystacks: string[]): Match[];
  matchListIndices(haystacks: string[]): MatchIndices[];
  setNeedle(needle: string): void;
  free(): void;
} {
  const inner = new (getWasm().Matcher)(needle, config ?? defaultConfig);
  return {
    matchList: (haystacks) => inner.matchList(haystacks) as Match[],
    matchListIndices: (haystacks) =>
      inner.matchListIndices(haystacks) as MatchIndices[],
    setNeedle: (needle) => inner.setNeedle(needle),
    free: () => inner.free(),
  };
}

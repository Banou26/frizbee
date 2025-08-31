// TypeScript definitions for Frizbee WASM package

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
    max_typos: number | null | undefined;
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

export class WasmMatcher {
    constructor();

    /**
     * Find matches for a needle in a list of haystacks
     * @param needle - The search term
     * @param haystacks - List of strings to search in
     * @param config - Optional configuration object
     * @returns Array of matches with scores and indices
     */
    matchList(needle: string, haystacks: string[], config?: Config): Match[];

    /**
     * Get character indices where the needle matches in the haystack
     * @param needle - The search term
     * @param haystack - The string to search in
     * @param config - Optional configuration object
     * @returns Match indices and score, or null if no match
     */
    matchIndices(needle: string, haystack: string, config?: Config): MatchIndices | null;

    /**
     * Compare all items against each other and return similarity scores
     * @param items - List of strings to compare
     * @param config - Optional configuration object
     * @param minScore - Minimum score threshold (default: 0)
     * @returns Array of comparison results sorted by score
     */
    compareAll(items: string[], config?: Config, minScore?: number): ComparisonResult[];
}

/**
 * Create a default configuration object
 * @returns Default configuration with standard settings
 */
export function create_default_config(): Config;

/**
 * Create a default scoring object
 * @returns Default scoring parameters
 */
export function create_default_scoring(): Scoring;

/**
 * Create a custom configuration object
 * @param prefilter - Enable prefiltering for performance
 * @param maxTypos - Maximum allowed typos (undefined for unlimited)
 * @param sort - Sort results by score
 * @param scoring - Custom scoring parameters (undefined for default)
 * @returns Custom configuration object
 */
export function create_custom_config(
    prefilter: boolean,
    maxTypos: number | null | undefined,
    sort: boolean,
    scoring?: Scoring
): Config;

/**
 * Create custom scoring parameters
 * @param matchScore - Score for matching character
 * @param mismatchPenalty - Penalty for mismatch
 * @param gapOpenPenalty - Penalty for opening gap
 * @param gapExtendPenalty - Penalty for extending gap
 * @param prefixBonus - Bonus for matching prefix
 * @param offsetPrefixBonus - Bonus for offset prefix
 * @param capitalizationBonus - Bonus for capitalization
 * @param matchingCaseBonus - Bonus for matching case
 * @param exactMatchBonus - Bonus for exact match
 * @param delimiterBonus - Bonus for delimiter
 * @param delimiters - String of delimiter characters
 * @returns Custom scoring object
 */
export function create_custom_scoring(
    matchScore?: number,
    mismatchPenalty?: number,
    gapOpenPenalty?: number,
    gapExtendPenalty?: number,
    prefixBonus?: number,
    offsetPrefixBonus?: number,
    capitalizationBonus?: number,
    matchingCaseBonus?: number,
    exactMatchBonus?: number,
    delimiterBonus?: number,
    delimiters?: string
): Scoring;

/**
 * Initialize the WASM module
 * @param input - Optional input for initialization
 * @returns Promise that resolves when WASM is loaded
 */
export default function init(input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module): Promise<void>;

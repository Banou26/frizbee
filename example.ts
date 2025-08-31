import init, { WasmMatcher, create_default_config, create_default_scoring } from './pkg/frizbee_wasm';

// TypeScript interfaces for configuration
interface Scoring {
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

interface Config {
    prefilter: boolean;
    max_typos: number | null;
    sort: boolean;
    scoring: Scoring;
}

interface Match {
    score: number;
    index: number;
    exact: boolean;
}

interface MatchIndices {
    score: number;
    indices: number[];
    exact: boolean;
}

async function main() {
    // Initialize the WASM module
    await init();

    // Create a matcher instance
    const matcher = new WasmMatcher();

    // Get default configuration
    const defaultConfig: Config = create_default_config();
    
    // Custom configuration example
    const customConfig: Config = {
        prefilter: true,
        max_typos: 2,
        sort: true,
        scoring: create_default_scoring()
    };

    // Sample data
    const files = [
        "src/components/Header.tsx",
        "src/components/Footer.tsx",
        "src/utils/helpers.ts",
        "package.json",
        "tsconfig.json",
        "README.md"
    ];

    // Search for matches
    const needle = "hdr";
    const matches: Match[] = matcher.matchList(needle, files, customConfig);

    // Display results
    console.log(`Searching for "${needle}":`);
    matches.forEach(match => {
        console.log(`- ${files[match.index]} (score: ${match.score}, exact: ${match.exact})`);
    });

    // Get match indices for highlighting
    const bestMatch = matches[0];
    if (bestMatch) {
        const haystack = files[bestMatch.index];
        const indices: MatchIndices | null = matcher.matchIndices(needle, haystack, customConfig);
        
        if (indices) {
            console.log(`\nMatch indices for "${haystack}":`, indices.indices);
            
            // Build highlighted string
            let highlighted = '';
            let lastIndex = 0;
            for (const idx of indices.indices) {
                highlighted += haystack.slice(lastIndex, idx);
                highlighted += `[${haystack[idx]}]`;
                lastIndex = idx + 1;
            }
            highlighted += haystack.slice(lastIndex);
            console.log(`Highlighted: ${highlighted}`);
        }
    }
}

// Run the example
main().catch(console.error);
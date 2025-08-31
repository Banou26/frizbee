// Example showing all ways to create custom configurations in Frizbee WASM

import init, { 
    WasmMatcher, 
    create_default_config,
    create_custom_config,
    create_custom_scoring
} from './pkg/frizbee_wasm.js';

async function main() {
    await init();
    
    const matcher = new WasmMatcher();
    
    // 1. Default configuration (strictest matching)
    const defaultConfig = create_default_config();
    // {
    //   prefilter: true,
    //   max_typos: 0,
    //   sort: true,
    //   scoring: { /* default scoring values */ }
    // }
    
    // 2. Simple custom config with default scoring
    const simpleConfig = create_custom_config(
        true,       // prefilter enabled
        2,          // allow up to 2 typos
        true,       // sort results by score
        undefined   // use default scoring
    );
    
    // 3. No typo filtering (most permissive)
    const permissiveConfig = create_custom_config(
        false,      // no prefiltering
        undefined,  // unlimited typos
        true,       // still sort results
        undefined   // default scoring
    );
    
    // 4. Custom scoring for file path matching
    const filePathScoring = create_custom_scoring(
        16,         // match_score
        4,          // mismatch_penalty
        6,          // gap_open_penalty
        2,          // gap_extend_penalty
        10,         // prefix_bonus (reward matching start of path)
        5,          // offset_prefix_bonus
        8,          // capitalization_bonus (for camelCase files)
        3,          // matching_case_bonus
        25,         // exact_match_bonus
        12,         // delimiter_bonus (high for path segments)
        "/\\._-"    // delimiters for file paths
    );
    
    const filePathConfig = create_custom_config(
        true,
        1,
        true,
        filePathScoring
    );
    
    // 5. Custom scoring for variable/function names
    const codeScoring = create_custom_scoring(
        16,         // match_score
        5,          // mismatch_penalty
        7,          // gap_open_penalty
        3,          // gap_extend_penalty
        8,          // prefix_bonus
        4,          // offset_prefix_bonus
        10,         // capitalization_bonus (important for camelCase)
        4,          // matching_case_bonus
        20,         // exact_match_bonus
        10,         // delimiter_bonus
        "_$"        // delimiters for code identifiers
    );
    
    const codeConfig = create_custom_config(
        true,
        2,
        true,
        codeScoring
    );
    
    // 6. Strict config (no typos, high penalties)
    const strictScoring = create_custom_scoring(
        20,         // high match score
        10,         // high mismatch penalty
        15,         // high gap open penalty
        8,          // high gap extend penalty
        15,         // high prefix bonus
        8,          // offset_prefix_bonus
        6,          // capitalization_bonus
        5,          // matching_case_bonus
        40,         // very high exact match bonus
        15,         // high delimiter bonus
        " /._-"     // standard delimiters
    );
    
    const strictConfig = create_custom_config(
        true,
        0,          // no typos allowed
        true,
        strictScoring
    );
    
    // 7. URL/Domain matching config
    const urlScoring = create_custom_scoring(
        16,         // match_score
        4,          // mismatch_penalty
        5,          // gap_open_penalty
        2,          // gap_extend_penalty
        12,         // high prefix bonus (domain names often typed from start)
        6,          // offset_prefix_bonus
        2,          // low capitalization_bonus (URLs typically lowercase)
        1,          // low matching_case_bonus
        30,         // exact_match_bonus
        15,         // high delimiter bonus
        "/.:-@"     // URL delimiters
    );
    
    const urlConfig = create_custom_config(
        true,
        1,
        true,
        urlScoring
    );
    
    // Example usage with different configs
    const items = [
        "src/components/Header.tsx",
        "src/components/Footer.tsx",
        "package.json",
        "getUserData",
        "setUserData",
        "updateUserProfile",
        "https://example.com/api",
        "https://api.example.com/v2",
        "README.md"
    ];
    
    const needle = "user";
    
    console.log("Default Config Results:");
    const defaultResults = matcher.matchList(needle, items, defaultConfig);
    console.log(defaultResults);
    
    console.log("\nPermissive Config Results:");
    const permissiveResults = matcher.matchList(needle, items, permissiveConfig);
    console.log(permissiveResults);
    
    console.log("\nCode Config Results (optimized for function names):");
    const codeResults = matcher.matchList(needle, items, codeConfig);
    console.log(codeResults);
    
    // You can also create configs dynamically
    function createConfigForContext(context) {
        switch(context) {
            case 'files':
                return filePathConfig;
            case 'code':
                return codeConfig;
            case 'urls':
                return urlConfig;
            case 'strict':
                return strictConfig;
            default:
                return defaultConfig;
        }
    }
    
    // Use appropriate config based on context
    const contextConfig = createConfigForContext('code');
    const contextResults = matcher.matchList(needle, items, contextConfig);
}

main().catch(console.error);
# Frizbee WASM Package

This is the WebAssembly version of Frizbee, a fast fuzzy matching library that uses SIMD Smith-Waterman algorithm similar to FZF/FZY.

## Building

Install wasm-pack if you haven't already:
```bash
cargo install wasm-pack
```

Build for web (browser):
```bash
wasm-pack build --target web --out-dir pkg --features wasm
```

Build for Node.js:
```bash
wasm-pack build --target nodejs --out-dir pkg-node --features wasm
```

Build for bundlers (webpack, rollup, etc):
```bash
wasm-pack build --target bundler --out-dir pkg-bundler --features wasm
```

## Usage in Browser

See `example.html` for a complete example. Basic usage:

```javascript
import init, { WasmMatcher, create_default_config } from './pkg/frizbee.js';

await init();

const matcher = new WasmMatcher();
const config = create_default_config();

const items = ["hello_world.js", "main.rs", "package.json"];
const needle = "hw";

const matches = matcher.matchList(needle, items, config);
// Returns array of matches with score, index, and exact flag
```

## Usage with TypeScript

See `example.ts` for TypeScript usage. The package includes TypeScript definitions.

```typescript
import init, { WasmMatcher, create_default_config } from './pkg/frizbee';

// TypeScript interfaces are provided for Config, Match, MatchIndices, etc.
```

## API

### WasmMatcher

- `new WasmMatcher()` - Create a new matcher instance
- `matchList(needle: string, haystacks: string[], config?: Config)` - Find matches in a list
- `matchIndices(needle: string, haystack: string, config?: Config)` - Get character indices of matches

### Configuration Functions

- `create_default_config()` - Get default configuration
- `create_default_scoring()` - Get default scoring parameters
- `create_custom_config(prefilter: boolean, max_typos?: number, sort: boolean, scoring?: Scoring)` - Create custom configuration
- `create_custom_scoring(...)` - Create custom scoring with specific parameters (all optional)

## Configuration

The `Config` object supports:
- `prefilter`: Enable prefiltering for performance
- `max_typos`: Maximum allowed typos (undefined/null for unlimited)
- `sort`: Sort results by score
- `scoring`: Scoring parameters object

### Creating Custom Configurations

```javascript
// Simple custom config
const config = create_custom_config(
    true,     // prefilter
    2,        // max_typos
    true,     // sort
    undefined // use default scoring
);

// Custom scoring parameters
const scoring = create_custom_scoring(
    16,       // match_score
    4,        // mismatch_penalty
    6,        // gap_open_penalty
    2,        // gap_extend_penalty
    8,        // prefix_bonus
    4,        // offset_prefix_bonus
    4,        // capitalization_bonus
    2,        // matching_case_bonus
    20,       // exact_match_bonus
    8,        // delimiter_bonus
    " /._-"   // delimiters
);

// Config with custom scoring
const advancedConfig = create_custom_config(true, 1, true, scoring);
```

See `custom-config-example.js` for comprehensive examples of different configurations optimized for various use cases (file paths, code identifiers, URLs, etc.)

## Notes

- The WASM build excludes parallel sorting features (rayon) as WASM is single-threaded
- SIMD instructions may not be available in all browsers/environments
- For best performance, use modern browsers with WASM SIMD support
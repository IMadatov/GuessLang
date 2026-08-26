# Text Analyzer (GuessLang) Design

## 1. Overview
The Text Analyzer is a universal library designed to determine whether a given text is a URL, code, and if it's code, identify the specific programming language. To ensure high performance, broad compatibility, and maintainability, the core logic will be written in Rust, utilizing a heuristics-based scoring approach, and exposing native bindings for Python, Node.js, and C# (.NET).

## 2. Core Architecture (Rust)
The core library, named `text_analyzer_core`, will expose a single main function: `analyze_text(text: &str) -> AnalysisResult`.

### 2.1 Output Structure
The result struct will contain:
- `is_url`: Boolean indicating if the text matches standard URL patterns.
- `is_code`: Boolean indicating if the text contains sufficient programming language characteristics.
- `language`: String literal (e.g., "python", "javascript", "csharp") or `null` if `is_code` is false.

### 2.2 Heuristics & Scoring Engine
- **URL Detection**: Standard regular expressions validating standard HTTP/HTTPS/FTP protocols and domain structures.
- **Language Detection (Scoring System)**:
  - Each supported language has a predefined set of keywords and syntactical tokens (e.g., Python: `def`, `import`, `:` ; JavaScript: `const`, `=>`, `console.log` ; C#: `public class`, `using`, `Task<>`).
  - The engine tokenizes the text and tallies scores for each language.
  - The language with the highest score above a predefined minimum threshold is selected.
  - If no language crosses the threshold, `is_code` evaluates to `false`.

## 3. Bindings & Interoperability
To ensure universal usability without network overhead, the Rust core will be compiled and bound to major runtimes:
- **Python**: Uses `PyO3` to compile a native Python extension.
- **Node.js**: Uses `napi-rs` to provide a fast V8-compatible binary.
- **.NET (C#)**: Exposes a standard C-ABI `extern "C"` interface. A .NET wrapper class will use `DllImport` to invoke the native binary.

## 4. Sub-projects & Implementation Scope
Due to the multi-language nature of this project, it must be implemented in the following isolated phases:
1. **Phase 1**: Setup Rust workspace, implement URL detection, and basic Language scoring engine with unit tests.
2. **Phase 2**: Implement Python bindings (`PyO3`).
3. **Phase 3**: Implement Node.js bindings (`napi-rs`).
4. **Phase 4**: Implement C# (.NET) FFI and wrapper library.

## 5. Error Handling
- The library will not panic on malformed UTF-8; it will handle string conversion errors gracefully and return `is_code: false`.
- FFI boundaries will use safe string pointer allocations and explicit memory freeing to prevent leaks.

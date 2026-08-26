# Text Analyzer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a Rust core library (`text_analyzer_core`) that detects if a text is a URL, code, and identifies the programming language using heuristics.

**Architecture:** A heuristics-based Rust library that scores text against language-specific keywords.

**Tech Stack:** Rust, Cargo, Regex.

## Global Constraints

- No panics on invalid input.
- Cross-platform compatibility.

---

### Task 1: Setup Rust Cargo Project & Output Struct

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `src/models.rs`
- Create: `tests/integration_test.rs`

**Interfaces:**
- Produces: `AnalysisResult` struct with `is_url`, `is_code`, and `language`.

- [ ] **Step 1: Write the failing test**

```rust
// tests/integration_test.rs
use text_analyzer_core::analyze_text;

#[test]
fn test_basic_struct() {
    let result = analyze_text("hello");
    assert_eq!(result.is_url, false);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL with compilation error (text_analyzer_core not found)

- [ ] **Step 3: Write minimal implementation**

```toml
# Cargo.toml
[package]
name = "text_analyzer_core"
version = "0.1.0"
edition = "2021"
```

```rust
// src/models.rs
#[derive(Debug, PartialEq)]
pub struct AnalysisResult {
    pub is_url: bool,
    pub is_code: bool,
    pub language: Option<String>,
}

// src/lib.rs
pub mod models;
use models::AnalysisResult;

pub fn analyze_text(_text: &str) -> AnalysisResult {
    AnalysisResult {
        is_url: false,
        is_code: false,
        language: None,
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/ tests/
git commit -m "feat: setup Rust core and output models"
```

### Task 2: URL Detection Implementation

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Create: `src/url_detector.rs`

**Interfaces:**
- Consumes: `AnalysisResult`
- Produces: `is_url` logic using `regex` crate.

- [ ] **Step 1: Write the failing test**

```rust
// tests/integration_test.rs
// (add to existing tests)
#[test]
fn test_url_detection() {
    let result1 = analyze_text("https://google.com");
    assert_eq!(result1.is_url, true);
    
    let result2 = analyze_text("just text");
    assert_eq!(result2.is_url, false);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL (assertion failed: result1.is_url == true)

- [ ] **Step 3: Write minimal implementation**

Add `regex = "1.10"` to dependencies in `Cargo.toml`.
```rust
// src/url_detector.rs
use regex::Regex;

pub fn is_url(text: &str) -> bool {
    let re = Regex::new(r"^(https?://|www\.)[^\s/$.?#].[^\s]*$").unwrap();
    re.is_match(text)
}

// src/lib.rs
pub mod models;
pub mod url_detector;
use models::AnalysisResult;

pub fn analyze_text(text: &str) -> AnalysisResult {
    AnalysisResult {
        is_url: url_detector::is_url(text),
        is_code: false,
        language: None,
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/ tests/
git commit -m "feat: implement URL detection using regex"
```

### Task 3: Code Language Heuristics Engine

**Files:**
- Modify: `src/lib.rs`
- Create: `src/language_detector.rs`

**Interfaces:**
- Produces: `language` and `is_code` values based on scoring.

- [ ] **Step 1: Write the failing test**

```rust
// tests/integration_test.rs
// (add to existing tests)
#[test]
fn test_python_code() {
    let code = "def hello():\n    print('world')";
    let result = analyze_text(code);
    assert_eq!(result.is_code, true);
    assert_eq!(result.language, Some("python".to_string()));
}

#[test]
fn test_csharp_code() {
    let code = "public class Hello {\n    public static void Main() { }\n}";
    let result = analyze_text(code);
    assert_eq!(result.is_code, true);
    assert_eq!(result.language, Some("csharp".to_string()));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL (assertion failed: result.is_code == true)

- [ ] **Step 3: Write minimal implementation**

```rust
// src/language_detector.rs
pub fn detect_language(text: &str) -> (bool, Option<String>) {
    let mut python_score = 0;
    let mut csharp_score = 0;
    
    if text.contains("def ") { python_score += 1; }
    if text.contains("print(") { python_score += 1; }
    
    if text.contains("public class") { csharp_score += 1; }
    if text.contains("public static void") { csharp_score += 1; }
    
    if python_score > 0 && python_score >= csharp_score {
        return (true, Some("python".to_string()));
    } else if csharp_score > 0 {
        return (true, Some("csharp".to_string()));
    }
    
    (false, None)
}

// src/lib.rs
pub mod models;
pub mod url_detector;
pub mod language_detector;
use models::AnalysisResult;

pub fn analyze_text(text: &str) -> AnalysisResult {
    let (is_code, language) = language_detector::detect_language(text);
    AnalysisResult {
        is_url: url_detector::is_url(text),
        is_code,
        language,
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/ tests/
git commit -m "feat: basic language detection heuristics for Python and C#"
```

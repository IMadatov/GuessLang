# Text Analyzer Implementation Plan v2 (Universal Types)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create an extensible Rust core library (`text_analyzer_core`) that detects various text types (Code, URLs, Dates, Numbers, JSON, etc.) and scores them to find the best match.

**Architecture:** A modular, plugin-like architecture in Rust. A central engine iterates through a registry of detectors (Regex-based or Heuristics-based) that implement a common `TypeDetector` trait. The detector returning the highest confidence score determines the text's data type.

**Tech Stack:** Rust, Cargo, `regex`, `serde_json` (for JSON validation).

## Global Constraints

- No panics on invalid input.
- High performance (compile regexes once using `lazy_static` or `once_cell`).
- Cross-platform compatibility.

---

### Task 1: Setup Rust Cargo Project & Extensible Architecture

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `src/models.rs`
- Create: `src/engine.rs`

**Interfaces:**
- Produces: `AnalysisResult` struct with `detected_type`, `sub_type`, and `confidence_score`.

- [ ] **Step 1: Write the failing test**

```rust
// tests/integration_test.rs
use text_analyzer_core::analyze_text;

#[test]
fn test_basic_struct() {
    let result = analyze_text("hello");
    assert_eq!(result.detected_type, "Unknown");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL (text_analyzer_core not found)

- [ ] **Step 3: Write minimal implementation**

```toml
# Cargo.toml
[package]
name = "text_analyzer_core"
version = "0.1.0"
edition = "2021"

[dependencies]
regex = "1.10"
once_cell = "1.19"
```

```rust
// src/models.rs
#[derive(Debug, PartialEq)]
pub struct AnalysisResult {
    pub detected_type: String,
    pub sub_type: Option<String>,
    pub confidence_score: f32,
}

pub trait TypeDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult>;
}

// src/engine.rs
use crate::models::{AnalysisResult, TypeDetector};

pub struct AnalyzerEngine {
    detectors: Vec<Box<dyn TypeDetector>>,
}

impl AnalyzerEngine {
    pub fn new() -> Self {
        Self { detectors: vec![] }
    }
    
    pub fn add_detector(&mut self, detector: Box<dyn TypeDetector>) {
        self.detectors.push(detector);
    }
    
    pub fn analyze(&self, text: &str) -> AnalysisResult {
        let mut best_result = AnalysisResult {
            detected_type: "Unknown".to_string(),
            sub_type: None,
            confidence_score: 0.0,
        };
        
        for detector in &self.detectors {
            if let Some(res) = detector.detect(text) {
                if res.confidence_score > best_result.confidence_score {
                    best_result = res;
                }
            }
        }
        best_result
    }
}

// src/lib.rs
pub mod models;
pub mod engine;

pub fn analyze_text(_text: &str) -> models::AnalysisResult {
    let engine = engine::AnalyzerEngine::new();
    // Detectors will be added here later
    engine.analyze(_text)
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat: setup extensible type detection engine"
```

### Task 2: Network & Web Detectors

**Files:**
- Create: `src/detectors/network.rs`
- Create: `src/detectors/mod.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `UrlDetector`, `EmailDetector`, `IpDetector`, `MacDetector`.

- [ ] **Step 1: Write the failing tests**

```rust
// tests/network_test.rs
use text_analyzer_core::analyze_text;

#[test]
fn test_network_types() {
    assert_eq!(analyze_text("https://example.com").detected_type, "URL");
    assert_eq!(analyze_text("test@example.com").detected_type, "Email");
    assert_eq!(analyze_text("192.168.1.1").detected_type, "IP Address");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL

- [ ] **Step 3: Write minimal implementation**

```rust
// Implement basic Regex detectors for URL, Email, and IP Address in src/detectors/network.rs
// Register them in src/lib.rs inside `analyze_text`
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat: add Network type detectors (URL, Email, IP)"
```

### Task 3: Data Formats & Crypto Detectors

**Files:**
- Create: `src/detectors/data_formats.rs`
- Create: `src/detectors/crypto.rs`
- Modify: `src/lib.rs`
- Modify: `Cargo.toml` (Add `serde_json` for JSON parsing)

**Interfaces:**
- Produces: `JsonDetector`, `XmlDetector`, `UuidDetector`, `HashDetector`.

- [ ] **Step 1: Write the failing tests**

```rust
// tests/formats_test.rs
use text_analyzer_core::analyze_text;

#[test]
fn test_data_formats() {
    assert_eq!(analyze_text("{\"key\":\"value\"}").detected_type, "JSON");
    assert_eq!(analyze_text("123e4567-e89b-12d3-a456-426614174000").detected_type, "UUID");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL

- [ ] **Step 3: Write minimal implementation**

```rust
// Add serde_json and uuid crates to Cargo.toml
// Implement JsonDetector by trying serde_json::from_str
// Implement UuidDetector using Regex or uuid crate
// Register them in lib.rs
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat: add Data Formats and Crypto detectors (JSON, UUID)"
```

### Task 4: Programming Code Detectors (Heuristics)

**Files:**
- Create: `src/detectors/code.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `CodeDetector` (scores for Python, C#, JS, Rust, SQL, Bash).

- [ ] **Step 1: Write the failing tests**

```rust
// tests/code_test.rs
use text_analyzer_core::analyze_text;

#[test]
fn test_code() {
    let python = analyze_text("def run():\n  pass");
    assert_eq!(python.detected_type, "Code");
    assert_eq!(python.sub_type, Some("Python".to_string()));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL

- [ ] **Step 3: Write minimal implementation**

```rust
// Implement CodeDetector using keyword frequency scoring (def, class, import, public, etc.)
// Return highest score > threshold.
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat: add Code detector with multi-language scoring"
```

### Task 5: Temporal, Numerical & System Detectors

**Files:**
- Create: `src/detectors/temporal.rs`
- Create: `src/detectors/numerical.rs`
- Create: `src/detectors/system.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `DateDetector`, `NumberDetector`, `SemVerDetector`.

- [ ] **Step 1: Write the failing tests**

```rust
// tests/misc_test.rs
use text_analyzer_core::analyze_text;

#[test]
fn test_misc() {
    assert_eq!(analyze_text("2023-10-15T12:00:00Z").detected_type, "Date");
    assert_eq!(analyze_text("-123.456").detected_type, "Number");
    assert_eq!(analyze_text("v1.2.3").detected_type, "SemVer");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL

- [ ] **Step 3: Write minimal implementation**

```rust
// Implement Regex-based parsing for Date (ISO8601), Number, and SemVer
// Register them in lib.rs
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat: add Date, Number, and System detectors"
```

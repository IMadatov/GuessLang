// tests/integration_test.rs
use text_analyzer_core::analyze_text;

#[test]
fn test_basic_struct() {
    let result = analyze_text("hello");
    assert_eq!(result.detected_type, "Unknown");
}

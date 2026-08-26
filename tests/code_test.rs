use text_analyzer_core::analyze_text;

#[test]
fn test_code_python() {
    let result = analyze_text("def run():\n  pass");
    assert_eq!(result.detected_type, "Code");
    assert_eq!(result.sub_type.as_deref(), Some("Python"));
}

#[test]
fn test_code_js() {
    let result = analyze_text("function foo() {\n  console.log('bar');\n}");
    assert_eq!(result.detected_type, "Code");
    assert_eq!(result.sub_type.as_deref(), Some("JavaScript"));
}

#[test]
fn test_code_rust() {
    let result = analyze_text("fn main() {\n  let mut x = 5;\n}");
    assert_eq!(result.detected_type, "Code");
    assert_eq!(result.sub_type.as_deref(), Some("Rust"));
}

#[test]
fn test_code_sql() {
    let result = analyze_text("SELECT id, name FROM users WHERE active = 1");
    assert_eq!(result.detected_type, "Code");
    assert_eq!(result.sub_type.as_deref(), Some("SQL"));
}

#[test]
fn test_code_bash() {
    let result = analyze_text("#!/bin/bash\nif [ -z \"$VAR\" ]; then\n  echo \"Empty\"\nfi");
    assert_eq!(result.detected_type, "Code");
    assert_eq!(result.sub_type.as_deref(), Some("Bash"));
}

#[test]
fn test_code_csharp() {
    let result = analyze_text("public class Program {\n  public static void Main() { }\n}");
    assert_eq!(result.detected_type, "Code");
    assert_eq!(result.sub_type.as_deref(), Some("C#"));
}

#[test]
fn test_code_negative() {
    let result = analyze_text("This is just some regular English text that mentions words like function and public but not enough to trigger.");
    // Wait, the regular text could have some keywords, we shouldn't trigger code detector if the score is too low.
    // Let's assert it's "Unknown" or some non-Code type (like maybe text? The engine returns "Unknown" by default)
    assert_ne!(result.detected_type, "Code");
}

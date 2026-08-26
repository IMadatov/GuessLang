use text_analyzer_core::analyze_text;

#[test]
fn test_misc_positive() {
    assert_eq!(analyze_text("2023-10-15T12:00:00Z").detected_type, "Date");
    assert_eq!(analyze_text("2023-10-15").detected_type, "Date");
    assert_eq!(analyze_text("-123.456").detected_type, "Number");
    assert_eq!(analyze_text("42").detected_type, "Number");
    assert_eq!(analyze_text("0x1A2B").detected_type, "Number");
    assert_eq!(analyze_text("v1.2.3").detected_type, "SemVer");
    assert_eq!(analyze_text("1.0.0-alpha.1").detected_type, "SemVer");
}

#[test]
fn test_misc_negative() {
    assert_ne!(analyze_text("Not a date 2023-10-15").detected_type, "Date");
    assert_ne!(analyze_text("123.456.789").detected_type, "Number");
    assert_ne!(analyze_text("v1.2.x").detected_type, "SemVer");
}

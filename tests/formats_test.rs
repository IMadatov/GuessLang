use text_analyzer_core::analyze_text;

#[test]
fn test_data_formats() {
    assert_eq!(analyze_text("{\"key\":\"value\"}").detected_type, "JSON");
    assert_eq!(analyze_text("123e4567-e89b-12d3-a456-426614174000").detected_type, "UUID");
    assert_eq!(analyze_text("<root><child>text</child></root>").detected_type, "XML");
    assert_eq!(analyze_text("<?xml version=\"1.0\"?><root/>").detected_type, "XML");
}

#[test]
fn test_data_formats_failures() {
    // Malformed JSON
    assert_ne!(analyze_text("{\"key\":\"value\"").detected_type, "JSON");
    
    // Invalid UUID
    assert_ne!(analyze_text("123e4567-e89b-12d3-a456-42661417400Z").detected_type, "UUID");
    
    // Malformed XML (does not match our strict regex)
    assert_ne!(analyze_text("<root><child>text</child>").detected_type, "XML");
}

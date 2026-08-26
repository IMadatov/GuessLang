use text_analyzer_core::analyze_text;

#[test]
fn test_network_types() {
    assert_eq!(analyze_text("https://example.com").detected_type, "URL");
    assert_eq!(analyze_text("test@example.com").detected_type, "Email");
    assert_eq!(analyze_text("192.168.1.1").detected_type, "IP Address");
    assert_eq!(analyze_text("00:1B:44:11:3A:B7").detected_type, "MAC Address");
}

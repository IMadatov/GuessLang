use text_analyzer_core::analyze_text;

#[test]
fn test_crypto() {
    let md5_res = analyze_text("d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(md5_res.detected_type, "Hash");
    assert_eq!(md5_res.sub_type.as_deref(), Some("MD5"));
    
    let sha1_res = analyze_text("da39a3ee5e6b4b0d3255bfef95601890afd80709");
    assert_eq!(sha1_res.detected_type, "Hash");
    assert_eq!(sha1_res.sub_type.as_deref(), Some("SHA1"));
    
    let sha256_res = analyze_text("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256_res.detected_type, "Hash");
    assert_eq!(sha256_res.sub_type.as_deref(), Some("SHA256"));
}

#[test]
fn test_crypto_failures() {
    // Invalid length hash
    let invalid_md5 = analyze_text("d41d8cd98f00b204e9800998ecf8427"); // 31 chars
    assert_ne!(invalid_md5.detected_type, "Hash");
    
    // Invalid characters
    let invalid_sha1 = analyze_text("ga39a3ee5e6b4b0d3255bfef95601890afd80709"); // 'g' is not hex
    assert_ne!(invalid_sha1.detected_type, "Hash");
}

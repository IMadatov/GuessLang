use regex::Regex;
use std::sync::OnceLock;
use crate::models::{TypeDetector, AnalysisResult};

pub struct HashDetector;

impl TypeDetector for HashDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static MD5_RE: OnceLock<Regex> = OnceLock::new();
        let md5_re = MD5_RE.get_or_init(|| Regex::new(r"^[a-fA-F0-9]{32}$").unwrap());
        
        static SHA1_RE: OnceLock<Regex> = OnceLock::new();
        let sha1_re = SHA1_RE.get_or_init(|| Regex::new(r"^[a-fA-F0-9]{40}$").unwrap());
        
        static SHA256_RE: OnceLock<Regex> = OnceLock::new();
        let sha256_re = SHA256_RE.get_or_init(|| Regex::new(r"^[a-fA-F0-9]{64}$").unwrap());

        if md5_re.is_match(text) {
            return Some(AnalysisResult {
                detected_type: "Hash".to_string(),
                sub_type: Some("MD5".to_string()),
                confidence_score: 1.0,
            });
        }
        
        if sha1_re.is_match(text) {
            return Some(AnalysisResult {
                detected_type: "Hash".to_string(),
                sub_type: Some("SHA1".to_string()),
                confidence_score: 1.0,
            });
        }
        
        if sha256_re.is_match(text) {
            return Some(AnalysisResult {
                detected_type: "Hash".to_string(),
                sub_type: Some("SHA256".to_string()),
                confidence_score: 1.0,
            });
        }
        
        None
    }
}

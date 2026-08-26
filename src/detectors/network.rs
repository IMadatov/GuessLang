use regex::Regex;
use std::sync::OnceLock;
use crate::models::{TypeDetector, AnalysisResult};

pub struct UrlDetector;
pub struct EmailDetector;
pub struct IpDetector;
pub struct MacDetector;

fn detect_with_regex(text: &str, re: &Regex, type_name: &str) -> Option<AnalysisResult> {
    if re.is_match(text) {
        Some(AnalysisResult {
            detected_type: type_name.to_string(),
            sub_type: None,
            confidence_score: 1.0,
        })
    } else {
        None
    }
}

impl TypeDetector for UrlDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap());
        detect_with_regex(text, re, "URL")
    }
}

impl TypeDetector for EmailDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
        detect_with_regex(text, re, "Email")
    }
}

impl TypeDetector for IpDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap());
        detect_with_regex(text, re, "IP Address")
    }
}

impl TypeDetector for MacDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap());
        detect_with_regex(text, re, "MAC Address")
    }
}

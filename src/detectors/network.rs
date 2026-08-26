use regex::Regex;
use std::sync::OnceLock;
use crate::models::{TypeDetector, AnalysisResult};
use crate::detectors::utils::detect_with_regex;

pub struct UrlDetector;
pub struct EmailDetector;
pub struct IpDetector;
pub struct MacDetector;



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

use regex::Regex;
use std::sync::OnceLock;
use crate::models::{TypeDetector, AnalysisResult};
use crate::detectors::utils::detect_with_regex;

pub struct SemVerDetector;

impl TypeDetector for SemVerDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^v?\d+\.\d+\.\d+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$").unwrap());
        detect_with_regex(text, re, "SemVer")
    }
}

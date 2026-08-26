use regex::Regex;
use std::sync::OnceLock;
use crate::models::{TypeDetector, AnalysisResult};
use crate::detectors::utils::detect_with_regex;

pub struct NumberDetector;

impl TypeDetector for NumberDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^-?\d+(\.\d+)?$|^0x[0-9a-fA-F]+$").unwrap());
        detect_with_regex(text, re, "Number")
    }
}

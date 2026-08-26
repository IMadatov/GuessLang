use regex::Regex;
use std::sync::OnceLock;
use crate::models::{TypeDetector, AnalysisResult};
use crate::detectors::utils::detect_with_regex;

pub struct DateDetector;

impl TypeDetector for DateDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        // Regex matches ISO 8601 date-time, basic date YYYY-MM-DD, or unix timestamp (digits only)
        // Since number detector also matches digits, Unix Timestamp might overlap, but task says
        // "produces DateDetector, NumberDetector, SemVerDetector". 
        // We will match standard ISO 8601 dates and basic dates.
        let re = RE.get_or_init(|| Regex::new(r"^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2}))?$").unwrap());
        detect_with_regex(text, re, "Date")
    }
}

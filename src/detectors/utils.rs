use regex::Regex;
use crate::models::AnalysisResult;

pub fn detect_with_regex(text: &str, re: &Regex, type_name: &str) -> Option<AnalysisResult> {
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

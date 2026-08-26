use regex::Regex;
use std::sync::OnceLock;
use crate::models::{TypeDetector, AnalysisResult};
use crate::detectors::utils::detect_with_regex;

pub struct JsonDetector;
pub struct XmlDetector;
pub struct UuidDetector;



impl TypeDetector for JsonDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
            if value.is_object() || value.is_array() {
                return Some(AnalysisResult {
                    detected_type: "JSON".to_string(),
                    sub_type: None,
                    confidence_score: 1.0,
                });
            }
        }
        None
    }
}

impl TypeDetector for XmlDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        let trimmed = text.trim();
        
        if !trimmed.starts_with('<') || !trimmed.ends_with('>') {
            return None;
        }
        
        if trimmed.starts_with("<?xml") {
            return Some(AnalysisResult {
                detected_type: "XML".to_string(),
                sub_type: None,
                confidence_score: 1.0,
            });
        }
        
        if let Some(end_of_first_tag) = trimmed.find('>') {
            let first_tag_content = &trimmed[1..end_of_first_tag];
            let root_tag_name = first_tag_content.split_whitespace().next().unwrap_or("");
            
            if !root_tag_name.is_empty() && !root_tag_name.contains('/') {
                let closing_tag = format!("</{}>", root_tag_name);
                if trimmed.ends_with(&closing_tag) {
                    return Some(AnalysisResult {
                        detected_type: "XML".to_string(),
                        sub_type: None,
                        confidence_score: 1.0,
                    });
                }
            } else if trimmed.ends_with("/>") {
                return Some(AnalysisResult {
                    detected_type: "XML".to_string(),
                    sub_type: None,
                    confidence_score: 1.0,
                });
            }
        }
        
        None
    }
}

impl TypeDetector for UuidDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap());
        detect_with_regex(text, re, "UUID")
    }
}

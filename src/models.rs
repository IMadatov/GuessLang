#[derive(Debug, PartialEq)]
pub struct AnalysisResult {
    pub detected_type: String,
    pub sub_type: Option<String>,
    pub confidence_score: f32,
}

pub trait TypeDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult>;
}

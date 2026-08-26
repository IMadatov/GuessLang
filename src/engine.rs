use crate::models::{AnalysisResult, TypeDetector};

pub struct AnalyzerEngine {
    detectors: Vec<Box<dyn TypeDetector>>,
}

impl AnalyzerEngine {
    pub fn new() -> Self {
        Self { detectors: vec![] }
    }
    
    pub fn add_detector(&mut self, detector: Box<dyn TypeDetector>) {
        self.detectors.push(detector);
    }
    
    pub fn analyze(&self, text: &str) -> AnalysisResult {
        let mut best_result = AnalysisResult {
            detected_type: "Unknown".to_string(),
            sub_type: None,
            confidence_score: 0.0,
        };
        
        for detector in &self.detectors {
            if let Some(res) = detector.detect(text) {
                if res.confidence_score > best_result.confidence_score {
                    best_result = res;
                }
            }
        }
        best_result
    }
}

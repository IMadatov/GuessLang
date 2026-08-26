pub mod models;
pub mod engine;
pub mod detectors;

pub fn analyze_text(_text: &str) -> models::AnalysisResult {
    let mut engine = engine::AnalyzerEngine::new();
    
    // Register network detectors
    engine.add_detector(Box::new(detectors::network::UrlDetector));
    engine.add_detector(Box::new(detectors::network::EmailDetector));
    engine.add_detector(Box::new(detectors::network::IpDetector));
    engine.add_detector(Box::new(detectors::network::MacDetector));
    
    engine.analyze(_text)
}

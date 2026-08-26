pub mod models;
pub mod engine;
pub mod detectors;
pub mod ffi;

pub fn analyze_text(_text: &str) -> models::AnalysisResult {
    let mut engine = engine::AnalyzerEngine::new();
    
    // Register network detectors
    engine.add_detector(Box::new(detectors::network::UrlDetector));
    engine.add_detector(Box::new(detectors::network::EmailDetector));
    engine.add_detector(Box::new(detectors::network::IpDetector));
    engine.add_detector(Box::new(detectors::network::MacDetector));
    
    // Register data format detectors
    engine.add_detector(Box::new(detectors::data_formats::JsonDetector));
    engine.add_detector(Box::new(detectors::data_formats::XmlDetector));
    engine.add_detector(Box::new(detectors::data_formats::UuidDetector));
    
    // Register crypto detectors
    engine.add_detector(Box::new(detectors::crypto::HashDetector));
    
    // Register code detectors
    engine.add_detector(Box::new(detectors::code::CodeDetector));
    
    // Register misc detectors
    engine.add_detector(Box::new(detectors::temporal::DateDetector));
    engine.add_detector(Box::new(detectors::numerical::NumberDetector));
    engine.add_detector(Box::new(detectors::system::SemVerDetector));
    
    engine.analyze(_text)
}

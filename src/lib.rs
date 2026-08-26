pub mod models;
pub mod engine;

pub fn analyze_text(_text: &str) -> models::AnalysisResult {
    let engine = engine::AnalyzerEngine::new();
    // Detectors will be added here later
    engine.analyze(_text)
}

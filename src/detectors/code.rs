use crate::models::{AnalysisResult, TypeDetector};
use regex::Regex;
use std::sync::OnceLock;

pub struct CodeDetector;

impl TypeDetector for CodeDetector {
    fn detect(&self, text: &str) -> Option<AnalysisResult> {
        let scores = [
            ("Python", score_python(text)),
            ("JavaScript", score_javascript(text)),
            ("Rust", score_rust(text)),
            ("SQL", score_sql(text)),
            ("Bash", score_bash(text)),
            ("C#", score_csharp(text)),
        ];

        let mut best_lang = None;
        let mut best_score = 0.0;

        for (lang, score) in scores.iter() {
            if *score > best_score && *score >= 0.5 {
                best_score = *score;
                best_lang = Some(*lang);
            }
        }

        best_lang.map(|lang| AnalysisResult {
            detected_type: "Code".to_string(),
            sub_type: Some(lang.to_string()),
            confidence_score: best_score,
        })
    }
}

fn calculate_score(text: &str, re: &Regex) -> f32 {
    let count = re.find_iter(text).take(3).count();
    match count {
        0 => 0.0,
        1 => 0.4,
        2 => 0.7,
        _ => 1.0,
    }
}

macro_rules! create_scorer {
    ($func_name:ident, $regex_str:expr) => {
        fn $func_name(text: &str) -> f32 {
            static RE: OnceLock<Regex> = OnceLock::new();
            let re = RE.get_or_init(|| Regex::new($regex_str).unwrap());
            calculate_score(text, re)
        }
    };
}

create_scorer!(score_python, r"(?m)(\bdef\s+|\bclass\s+|\bimport\s+|\bfrom\s+.*\s+import\b|\belif\b|\bexcept\b|\bpass\b|\byield\b|\bprint\s*\(|\b__init__\b)");
create_scorer!(score_javascript, r"(\bfunction\s+|\bconst\s+|\blet\s+|\bvar\s+|\bconsole\.log\s*\(|=>|\bexport\s+|\bimport\s+.*\s+from\b|\basync\s+function|\bawait\s+)");
create_scorer!(score_rust, r"(\bfn\s+|\blet\s+mut\b|\bpub\s+(fn|struct|enum|trait|impl)\b|\bimpl\s+|\btrait\s+|\bmatch\s+|\buse\s+crate\b|\benum\s+|\bstruct\s+)");
create_scorer!(score_sql, r"(\bSELECT\b|\bFROM\b|\bWHERE\b|\bINSERT\s+INTO\b|\bUPDATE\b|\bDELETE\s+FROM\b|\bJOIN\b|\bGROUP\s+BY\b|\bORDER\s+BY\b)");
create_scorer!(score_bash, r"(?m)(#!/bin/bash|#!/bin/sh|\bif\s+\[|\bfi\b|\besac\b|\bchmod\s+[0-7]{3}\b|\bchown\s+)");
create_scorer!(score_csharp, r"(\bpublic\s+class\b|\bnamespace\s+|\busing\s+System\b|\bvoid\s+|\bTask<|\bIActionResult\b|\bConsole\.WriteLine\b)");

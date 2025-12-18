//! Content analysis for interview answers
//! Evaluates logic, job match, and keyword coverage

use anyhow::Result;
use std::collections::HashSet;

/// Content analyzer for interview answers
pub struct ContentAnalyzer;

/// Analysis result with multiple dimensions
#[derive(Debug, Clone)]
pub struct ContentAnalysisResult {
    pub logic_score: f32,            // 1-10: Answer coherence and logic
    pub job_match_score: f32,        // 1-10: Relevance to job requirements
    pub keyword_coverage: f32,       // 0-100: Keyword coverage percentage
    pub strengths: Vec<String>,      // Key strengths identified
    pub weaknesses: Vec<String>,     // Areas for improvement
}

impl ContentAnalyzer {
    /// Analyze answer content across multiple dimensions
    pub fn analyze(
        answer: &str,
        _question: &str,
        job_description: &str,
    ) -> Result<ContentAnalysisResult> {
        let logic_score = Self::evaluate_logic(answer);
        let job_match_score = Self::evaluate_job_match(answer, job_description);
        let keyword_coverage = Self::evaluate_keyword_coverage(answer, job_description);
        let strengths = Self::identify_strengths(answer, logic_score, job_match_score);
        let weaknesses = Self::identify_weaknesses(answer, logic_score, job_match_score);

        Ok(ContentAnalysisResult {
            logic_score,
            job_match_score,
            keyword_coverage,
            strengths,
            weaknesses,
        })
    }

    /// Evaluate answer logic and coherence (1-10)
    fn evaluate_logic(answer: &str) -> f32 {
        let answer_lower = answer.to_lowercase();
        let mut score = 5.0;

        // Structure indicators
        let structure_keywords = [
            "first", "second", "third", "finally", "in addition",
            "furthermore", "however", "moreover", "therefore",
            "首先", "其次", "最后", "此外", "然而", "因此"
        ];

        let mut structure_count = 0;
        for keyword in structure_keywords.iter() {
            if answer_lower.contains(keyword) {
                structure_count += 1;
            }
        }

        score += (structure_count as f32) * 0.8;

        // Length evaluation (appropriate length indicates thought)
        let word_count = answer.split_whitespace().count();
        if word_count >= 100 && word_count <= 500 {
            score += 1.5;
        } else if word_count > 500 {
            score -= 0.5;
        }

        // Avoid rambling (multiple consecutive punctuation)
        let rambling_indicators = answer.matches("...").count()
            + answer.matches("？？").count()
            + answer.matches(",,").count();
        score -= (rambling_indicators as f32) * 0.5;

        score.max(1.0).min(10.0)
    }

    /// Evaluate job relevance (1-10)
    fn evaluate_job_match(answer: &str, job_description: &str) -> f32 {
        let answer_lower = answer.to_lowercase();
        let jd_lower = job_description.to_lowercase();

        // Extract key terms from job description
        let mut jd_terms: HashSet<&str> = HashSet::new();
        for word in jd_lower.split_whitespace() {
            if word.len() > 3 && !Self::is_common_word(word) {
                jd_terms.insert(word.trim_matches(|c| !char::is_alphanumeric(c)));
            }
        }

        // Count matches
        let mut matched_count = 0;
        for term in &jd_terms {
            if answer_lower.contains(term) {
                matched_count += 1;
            }
        }

        let match_ratio = if jd_terms.is_empty() {
            0.5
        } else {
            matched_count as f32 / jd_terms.len() as f32
        };

        let base_score = match_ratio * 8.0 + 2.0;
        base_score.min(10.0)
    }

    /// Evaluate keyword coverage percentage (0-100)
    fn evaluate_keyword_coverage(answer: &str, job_description: &str) -> f32 {
        let answer_lower = answer.to_lowercase();
        let jd_lower = job_description.to_lowercase();

        // Extract important keywords from JD (length > 3, not common)
        let jd_keywords: Vec<&str> = jd_lower
            .split_whitespace()
            .filter(|w| {
                w.len() > 3 && !Self::is_common_word(w)
            })
            .collect();

        if jd_keywords.is_empty() {
            return 0.0;
        }

        let mut covered = 0;
        for kw in &jd_keywords {
            if answer_lower.contains(kw) {
                covered += 1;
            }
        }

        (covered as f32 / jd_keywords.len() as f32) * 100.0
    }

    /// Identify answer strengths
    fn identify_strengths(answer: &str, logic_score: f32, job_match_score: f32) -> Vec<String> {
        let mut strengths = Vec::new();

        if logic_score > 7.0 {
            strengths.push("逻辑清晰，结构完整".to_string());
        }

        if job_match_score > 7.0 {
            strengths.push("高度匹配岗位要求".to_string());
        }

        if answer.len() > 200 {
            strengths.push("答案详细具体".to_string());
        }

        let answer_lower = answer.to_lowercase();
        if answer_lower.contains("example") || answer_lower.contains("例如") || answer_lower.contains("具体") {
            strengths.push("提供了具体例子".to_string());
        }

        if strengths.is_empty() {
            strengths.push("有待改进".to_string());
        }

        strengths
    }

    /// Identify answer weaknesses
    fn identify_weaknesses(answer: &str, logic_score: f32, job_match_score: f32) -> Vec<String> {
        let mut weaknesses = Vec::new();

        if logic_score < 5.0 {
            weaknesses.push("逻辑不够清晰，需要更好的结构".to_string());
        }

        if job_match_score < 5.0 {
            weaknesses.push("与岗位要求的匹配度有限".to_string());
        }

        if answer.len() < 50 {
            weaknesses.push("答案过于简洁，缺乏详细信息".to_string());
        }

        let answer_lower = answer.to_lowercase();
        if !answer_lower.contains("example") && !answer_lower.contains("例如") {
            weaknesses.push("缺少具体例子或案例支撑".to_string());
        }

        if answer.matches(',').count() + answer.matches('，').count() < 2 {
            weaknesses.push("缺乏充分的展开和论述".to_string());
        }

        weaknesses
    }

    /// Check if word is common (should be filtered out)
    fn is_common_word(word: &str) -> bool {
        let common = [
            "the", "a", "an", "and", "or", "is", "are", "be", "to", "in", "on", "at",
            "of", "for", "with", "that", "this", "it", "by", "from", "as", "was",
            "在", "的", "了", "是", "我", "有", "不", "一", "个", "为", "以",
        ];
        common.contains(&word) || word.len() <= 2
    }
}

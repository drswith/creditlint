use std::path::PathBuf;

use regex::Regex;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub rules: Vec<Rule>,
    pub allowed_provenance_keys: Vec<String>,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            rules: vec![
                Rule {
                    id: "forbidden-ai-coauthor".to_string(),
                    kind: RuleKind::ForbiddenTrailer,
                    field_matcher: FieldMatcher::Exact("Co-authored-by".to_string()),
                    value_matcher: ValueMatcher::Pattern(
                        "(?i)(codex|claude|cursor|copilot|openai|anthropic|gemini|ai)".to_string(),
                    ),
                    message: "AI/tool authorship marker is not allowed".to_string(),
                },
                Rule {
                    id: "forbidden-made-with-marker".to_string(),
                    kind: RuleKind::FreeformMarker,
                    field_matcher: FieldMatcher::Any,
                    value_matcher: ValueMatcher::Pattern("(?i)^made[- ]with\\b.*$".to_string()),
                    message: "Made-with credit marker is not allowed".to_string(),
                },
                Rule {
                    id: "forbidden-made-on-marker".to_string(),
                    kind: RuleKind::FreeformMarker,
                    field_matcher: FieldMatcher::Any,
                    value_matcher: ValueMatcher::Pattern("(?i)^made[- ]on\\b.*$".to_string()),
                    message: "Made-on credit marker is not allowed".to_string(),
                },
                Rule {
                    id: "forbidden-generated-with-marker".to_string(),
                    kind: RuleKind::FreeformMarker,
                    field_matcher: FieldMatcher::Any,
                    value_matcher: ValueMatcher::Pattern(
                        "(?i)^generated[- ]with\\b.*$".to_string(),
                    ),
                    message: "Generated-with credit marker is not allowed".to_string(),
                },
            ],
            allowed_provenance_keys: vec![
                "AI-Assisted".to_string(),
                "Tool-Used".to_string(),
                "Generated-by".to_string(),
            ],
        }
    }
}

impl Policy {
    pub fn analyze(&self, source: Source, message: &str) -> Result<Vec<Violation>, AnalysisError> {
        let mut violations = Vec::new();

        for (line_index, raw_line) in message.lines().enumerate() {
            let line_number = line_index + 1;
            let trimmed = raw_line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if let Some((field, value)) = parse_trailer_line(trimmed) {
                for rule in self
                    .rules
                    .iter()
                    .filter(|rule| rule.kind == RuleKind::ForbiddenTrailer)
                {
                    if rule.matches_trailer(field, value)? {
                        violations.push(Violation {
                            source: source.clone(),
                            rule_id: rule.id.clone(),
                            field: Some(field.to_string()),
                            line: Some(line_number),
                            message: rule.message.clone(),
                        });
                    }
                }
            }

            for rule in self
                .rules
                .iter()
                .filter(|rule| rule.kind == RuleKind::FreeformMarker)
            {
                if rule.matches_freeform(trimmed)? {
                    violations.push(Violation {
                        source: source.clone(),
                        rule_id: rule.id.clone(),
                        field: None,
                        line: Some(line_number),
                        message: rule.message.clone(),
                    });
                }
            }
        }

        Ok(violations)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub kind: RuleKind,
    pub field_matcher: FieldMatcher,
    pub value_matcher: ValueMatcher,
    pub message: String,
}

impl Rule {
    fn matches_trailer(&self, field: &str, value: &str) -> Result<bool, AnalysisError> {
        Ok(self.field_matcher.matches(field, &self.id)?
            && self.value_matcher.matches(value, &self.id)?)
    }

    fn matches_freeform(&self, line: &str) -> Result<bool, AnalysisError> {
        match (&self.field_matcher, &self.value_matcher) {
            (FieldMatcher::Any, matcher) => matcher.matches(line, &self.id),
            (matcher, ValueMatcher::Any) => matcher.matches(line, &self.id),
            (field_matcher, value_matcher) => {
                Ok(field_matcher.matches(line, &self.id)?
                    && value_matcher.matches(line, &self.id)?)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    ForbiddenTrailer,
    FreeformMarker,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldMatcher {
    Any,
    Exact(String),
    Pattern(String),
}

impl FieldMatcher {
    fn matches(&self, value: &str, rule_id: &str) -> Result<bool, AnalysisError> {
        match self {
            FieldMatcher::Any => Ok(true),
            FieldMatcher::Exact(expected) => Ok(expected == value),
            FieldMatcher::Pattern(pattern) => matches_pattern(pattern, value, rule_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueMatcher {
    Any,
    Exact(String),
    Pattern(String),
}

impl ValueMatcher {
    fn matches(&self, value: &str, rule_id: &str) -> Result<bool, AnalysisError> {
        match self {
            ValueMatcher::Any => Ok(true),
            ValueMatcher::Exact(expected) => Ok(expected == value),
            ValueMatcher::Pattern(pattern) => matches_pattern(pattern, value, rule_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub kind: SourceKind,
    pub path: Option<PathBuf>,
    pub commit_sha: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    MessageFile,
    Stdin,
    Commit,
    Audit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub source: Source,
    pub rule_id: String,
    pub field: Option<String>,
    pub line: Option<usize>,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("invalid regex pattern in rule {rule_id}: {pattern}")]
    InvalidPattern {
        rule_id: String,
        pattern: String,
        #[source]
        source: regex::Error,
    },
}

fn parse_trailer_line(line: &str) -> Option<(&str, &str)> {
    let (field, value) = line.split_once(':')?;
    let field = field.trim();
    let value = value.trim();

    if field.is_empty() || value.is_empty() {
        return None;
    }

    Some((field, value))
}

fn matches_pattern(pattern: &str, value: &str, rule_id: &str) -> Result<bool, AnalysisError> {
    let regex = Regex::new(pattern).map_err(|source| AnalysisError::InvalidPattern {
        rule_id: rule_id.to_string(),
        pattern: pattern.to_string(),
        source,
    })?;

    Ok(regex.is_match(value))
}

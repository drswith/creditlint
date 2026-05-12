use std::path::PathBuf;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub kind: RuleKind,
    pub field_matcher: FieldMatcher,
    pub value_matcher: ValueMatcher,
    pub message: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueMatcher {
    Any,
    Exact(String),
    Pattern(String),
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

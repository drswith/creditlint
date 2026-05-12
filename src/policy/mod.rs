use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub rules: Vec<Rule>,
    pub allowed_provenance_keys: Vec<String>,
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

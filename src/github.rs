use regex::escape;
use thiserror::Error;

use crate::policy::{FieldMatcher, Policy, Rule, RuleKind, ValueMatcher};

pub fn export_ruleset_pattern(policy: &Policy) -> Result<String, RulesetExportError> {
    for key in &policy.allowed_provenance_keys {
        if policy
            .rules
            .iter()
            .any(|rule| forbidden_rule_targets_allowed_key(rule, key))
        {
            return Err(RulesetExportError::OverlappingAllowedProvenanceKey { key: key.clone() });
        }
    }

    let mut parts = Vec::with_capacity(policy.rules.len());
    for rule in &policy.rules {
        parts.push(export_rule(rule)?);
    }

    Ok(parts.join("|"))
}

fn forbidden_rule_targets_allowed_key(rule: &Rule, key: &str) -> bool {
    rule.kind == RuleKind::ForbiddenTrailer
        && matches!(&rule.field_matcher, FieldMatcher::Exact(field) if field == key)
}

fn export_rule(rule: &Rule) -> Result<String, RulesetExportError> {
    match rule.kind {
        RuleKind::ForbiddenTrailer => export_forbidden_trailer(rule),
        RuleKind::FreeformMarker => export_freeform_marker(rule),
    }
}

fn export_forbidden_trailer(rule: &Rule) -> Result<String, RulesetExportError> {
    let field = match &rule.field_matcher {
        FieldMatcher::Exact(field) => escape(field),
        _ => {
            return Err(RulesetExportError::UnsupportedRule {
                rule_id: rule.id.clone(),
                detail: "forbidden trailer export requires an exact trailer key".to_string(),
            });
        }
    };

    let value = export_value_matcher(rule, &rule.value_matcher)?;
    Ok(format!(r"(?:^|\n){}:\s*(?:{})(?:\n|$)", field, value))
}

fn export_freeform_marker(rule: &Rule) -> Result<String, RulesetExportError> {
    let pattern = match (&rule.field_matcher, &rule.value_matcher) {
        (FieldMatcher::Any, ValueMatcher::Pattern(pattern)) => pattern.as_str(),
        (FieldMatcher::Pattern(pattern), ValueMatcher::Any) => pattern.as_str(),
        _ => {
            return Err(RulesetExportError::UnsupportedRule {
                rule_id: rule.id.clone(),
                detail: "free-form export requires a single anchored line pattern".to_string(),
            });
        }
    };

    let normalized = normalize_pattern(pattern, &rule.id)?;
    if !normalized.anchored_start || !normalized.anchored_end {
        return Err(RulesetExportError::UnsupportedRule {
            rule_id: rule.id.clone(),
            detail: "free-form export requires ^...$ line anchors".to_string(),
        });
    }

    Ok(wrap_line_body(&normalized))
}

fn export_value_matcher(rule: &Rule, matcher: &ValueMatcher) -> Result<String, RulesetExportError> {
    match matcher {
        ValueMatcher::Any => Ok(".*".to_string()),
        ValueMatcher::Exact(value) => Ok(escape(value)),
        ValueMatcher::Pattern(pattern) => {
            let normalized = normalize_pattern(pattern, &rule.id)?;
            if normalized.anchored_start || normalized.anchored_end {
                return Err(RulesetExportError::UnsupportedRule {
                    rule_id: rule.id.clone(),
                    detail: "trailer value patterns must not use ^ or $ anchors".to_string(),
                });
            }

            Ok(apply_case_flag(&normalized))
        }
    }
}

fn wrap_line_body(pattern: &NormalizedPattern) -> String {
    format!(r"(?:^|\n)(?:{})(?:\n|$)", apply_case_flag(pattern))
}

fn apply_case_flag(pattern: &NormalizedPattern) -> String {
    if pattern.case_insensitive {
        format!("(?i:{})", pattern.body)
    } else {
        pattern.body.clone()
    }
}

fn normalize_pattern(
    pattern: &str,
    rule_id: &str,
) -> Result<NormalizedPattern, RulesetExportError> {
    let (case_insensitive, body) = if let Some(stripped) = pattern.strip_prefix("(?i)") {
        (true, stripped)
    } else {
        (false, pattern)
    };

    if body.contains("(?") {
        return Err(RulesetExportError::UnsupportedRule {
            rule_id: rule_id.to_string(),
            detail: "inline regex flags beyond a leading (?i) are not supported".to_string(),
        });
    }

    let anchored_start = body.starts_with('^');
    let anchored_end = body.ends_with('$');
    let body = body
        .strip_prefix('^')
        .unwrap_or(body)
        .strip_suffix('$')
        .unwrap_or(body)
        .to_string();

    Ok(NormalizedPattern {
        case_insensitive,
        anchored_start,
        anchored_end,
        body,
    })
}

struct NormalizedPattern {
    case_insensitive: bool,
    anchored_start: bool,
    anchored_end: bool,
    body: String,
}

#[derive(Debug, Error)]
pub enum RulesetExportError {
    #[error(
        "policy cannot be represented safely as a single GitHub ruleset regex because allowed provenance key `{key}` overlaps a forbidden rule; use merge-bot validation for final squash messages"
    )]
    OverlappingAllowedProvenanceKey { key: String },
    #[error(
        "policy cannot be represented safely as a single GitHub ruleset regex for rule `{rule_id}`: {detail}; use merge-bot validation for final squash messages"
    )]
    UnsupportedRule { rule_id: String, detail: String },
}

#[cfg(test)]
mod tests {
    use super::export_ruleset_pattern;
    use crate::policy::{FieldMatcher, Policy, Rule, RuleKind, ValueMatcher};

    #[test]
    fn exports_default_policy_to_single_regex() {
        let pattern = export_ruleset_pattern(&Policy::default()).expect("export pattern");

        assert!(pattern.contains("Co\\-authored\\-by"));
        assert!(pattern.contains("made[- ]with"));
        assert!(pattern.contains("generated[- ]with"));
    }

    #[test]
    fn fails_when_allowed_provenance_key_overlaps_forbidden_rule() {
        let policy = Policy {
            rules: vec![Rule {
                id: "generated-by-block".to_string(),
                kind: RuleKind::ForbiddenTrailer,
                field_matcher: FieldMatcher::Exact("Generated-by".to_string()),
                value_matcher: ValueMatcher::Any,
                message: "no generated-by".to_string(),
            }],
            identity_rules: vec![],
            allowed_provenance_keys: vec!["Generated-by".to_string()],
        };

        let error = export_ruleset_pattern(&policy).expect_err("overlap should fail");
        assert!(error.to_string().contains("cannot be represented safely"));
    }

    #[test]
    fn fails_for_regex_matched_trailer_field_names() {
        let policy = Policy {
            rules: vec![Rule {
                id: "regex-trailer-field".to_string(),
                kind: RuleKind::ForbiddenTrailer,
                field_matcher: FieldMatcher::Pattern("(?i)x-.*".to_string()),
                value_matcher: ValueMatcher::Pattern("agent".to_string()),
                message: "no regex keys".to_string(),
            }],
            identity_rules: vec![],
            allowed_provenance_keys: vec![],
        };

        let error = export_ruleset_pattern(&policy).expect_err("regex field should fail");
        assert!(error.to_string().contains("requires an exact trailer key"));
    }

    #[test]
    fn fails_for_anchored_trailer_value_patterns() {
        let policy = Policy {
            rules: vec![Rule {
                id: "anchored-trailer-value".to_string(),
                kind: RuleKind::ForbiddenTrailer,
                field_matcher: FieldMatcher::Exact("Co-authored-by".to_string()),
                value_matcher: ValueMatcher::Pattern("(?i)^codex$".to_string()),
                message: "no anchored values".to_string(),
            }],
            identity_rules: vec![],
            allowed_provenance_keys: vec![],
        };

        let error = export_ruleset_pattern(&policy).expect_err("anchored value should fail");
        assert!(error.to_string().contains("must not use ^ or $ anchors"));
    }

    #[test]
    fn fails_for_unanchored_freeform_patterns() {
        let policy = Policy {
            rules: vec![Rule {
                id: "freeform-prose".to_string(),
                kind: RuleKind::FreeformMarker,
                field_matcher: FieldMatcher::Any,
                value_matcher: ValueMatcher::Pattern("(?i)made with".to_string()),
                message: "no prose export".to_string(),
            }],
            identity_rules: vec![],
            allowed_provenance_keys: vec![],
        };

        let error = export_ruleset_pattern(&policy).expect_err("unanchored freeform should fail");
        assert!(error.to_string().contains("requires ^...$ line anchors"));
    }

    #[test]
    fn fails_for_extra_inline_regex_flags() {
        let policy = Policy {
            rules: vec![Rule {
                id: "inline-flags".to_string(),
                kind: RuleKind::FreeformMarker,
                field_matcher: FieldMatcher::Any,
                value_matcher: ValueMatcher::Pattern("(?i)(?m)^made with$".to_string()),
                message: "no inline flags".to_string(),
            }],
            identity_rules: vec![],
            allowed_provenance_keys: vec![],
        };

        let error = export_ruleset_pattern(&policy).expect_err("extra flags should fail");
        assert!(
            error
                .to_string()
                .contains("inline regex flags beyond a leading (?i) are not supported")
        );
    }
}

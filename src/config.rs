use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

use crate::policy::{FieldMatcher, Policy, Rule, RuleKind, ValueMatcher};

const CONFIG_FILE_NAME: &str = ".creditlint.yml";

pub fn load_policy_from_current_dir() -> Result<Policy, ConfigError> {
    let current_dir = env::current_dir().map_err(ConfigError::CurrentDir)?;
    load_policy(&current_dir)
}

pub fn load_policy(start_dir: &Path) -> Result<Policy, ConfigError> {
    let config_path = discover_config(start_dir)?;

    match config_path {
        Some(path) => {
            let content = fs::read_to_string(&path).map_err(|source| ConfigError::Read {
                path: path.clone(),
                source,
            })?;
            let raw = serde_yaml::from_str::<RawConfig>(&content).map_err(|source| {
                ConfigError::Parse {
                    path: path.clone(),
                    source,
                }
            })?;

            raw.into_policy()
        }
        None => Ok(Policy::default()),
    }
}

fn discover_config(start_dir: &Path) -> Result<Option<PathBuf>, ConfigError> {
    let repo_root = find_repo_root(start_dir)?;
    let mut current = Some(start_dir);

    while let Some(dir) = current {
        let candidate = dir.join(CONFIG_FILE_NAME);
        if candidate.is_file() {
            return Ok(Some(candidate));
        }

        if dir == repo_root {
            break;
        }

        current = dir.parent();
    }

    Ok(None)
}

fn find_repo_root(start_dir: &Path) -> Result<&Path, ConfigError> {
    let mut current = Some(start_dir);

    while let Some(dir) = current {
        let git_entry = dir.join(".git");
        if git_entry.exists() {
            return Ok(dir);
        }

        current = dir.parent();
    }

    Err(ConfigError::RepoRootNotFound {
        start_dir: start_dir.to_path_buf(),
    })
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to determine current working directory")]
    CurrentDir(#[source] std::io::Error),
    #[error("failed to read config file at {path}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse config file at {path}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("repository root could not be determined from {start_dir}")]
    RepoRootNotFound { start_dir: PathBuf },
    #[error("config validation failed: {0}")]
    Validation(String),
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    version: u32,
    rules: RawRules,
}

impl RawConfig {
    fn into_policy(self) -> Result<Policy, ConfigError> {
        if self.version != 1 {
            return Err(ConfigError::Validation(format!(
                "unsupported config version {}",
                self.version
            )));
        }

        let mut rules = Vec::with_capacity(self.rules.forbidden_trailers.len());

        for (index, raw_rule) in self.rules.forbidden_trailers.into_iter().enumerate() {
            rules.push(raw_rule.into_rule(index)?);
        }

        Ok(Policy {
            rules,
            allowed_provenance_keys: self.rules.allowed_provenance_trailers,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawRules {
    forbidden_trailers: Vec<RawForbiddenRule>,
    #[serde(default)]
    allowed_provenance_trailers: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawForbiddenRule {
    key: Option<String>,
    key_pattern: Option<String>,
    value_pattern: Option<String>,
}

impl RawForbiddenRule {
    fn into_rule(self, index: usize) -> Result<Rule, ConfigError> {
        let field_matcher = match (self.key, self.key_pattern) {
            (Some(key), None) => FieldMatcher::Exact(key),
            (None, Some(pattern)) => FieldMatcher::Pattern(pattern),
            (Some(_), Some(_)) => {
                return Err(ConfigError::Validation(format!(
                    "forbidden_trailers[{index}] cannot define both key and key_pattern"
                )));
            }
            (None, None) => {
                return Err(ConfigError::Validation(format!(
                    "forbidden_trailers[{index}] must define key or key_pattern"
                )));
            }
        };

        let kind = match (&field_matcher, self.value_pattern.as_ref()) {
            (FieldMatcher::Exact(_), _) => RuleKind::ForbiddenTrailer,
            (FieldMatcher::Pattern(_), Some(_)) => RuleKind::ForbiddenTrailer,
            (FieldMatcher::Pattern(_), None) => RuleKind::FreeformMarker,
            (FieldMatcher::Any, _) => RuleKind::FreeformMarker,
        };

        let value_matcher = match self.value_pattern {
            Some(pattern) => ValueMatcher::Pattern(pattern),
            None => ValueMatcher::Any,
        };

        validate_matcher(&field_matcher, index, "field")?;
        validate_matcher_value(&value_matcher, index, "value")?;

        Ok(Rule {
            id: format!("config-forbidden-rule-{index}"),
            kind,
            field_matcher,
            value_matcher,
            message: "Config-defined credit marker is not allowed".to_string(),
        })
    }
}

fn validate_matcher(
    matcher: &FieldMatcher,
    index: usize,
    matcher_name: &str,
) -> Result<(), ConfigError> {
    match matcher {
        FieldMatcher::Pattern(pattern) => validate_pattern(pattern, index, matcher_name),
        FieldMatcher::Any | FieldMatcher::Exact(_) => Ok(()),
    }
}

fn validate_matcher_value(
    matcher: &ValueMatcher,
    index: usize,
    matcher_name: &str,
) -> Result<(), ConfigError> {
    match matcher {
        ValueMatcher::Pattern(pattern) => validate_pattern(pattern, index, matcher_name),
        ValueMatcher::Any | ValueMatcher::Exact(_) => Ok(()),
    }
}

fn validate_pattern(pattern: &str, index: usize, matcher_name: &str) -> Result<(), ConfigError> {
    Regex::new(pattern).map_err(|source| {
        ConfigError::Validation(format!(
            "forbidden_trailers[{index}] has invalid {matcher_name}_pattern `{pattern}`: {source}"
        ))
    })?;

    Ok(())
}

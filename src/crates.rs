use std::fmt;

use chrono::{DateTime, Utc};

use super::*;

pub(super) trait CrateResponseExt {
    fn description(&self) -> &str;
    fn documentation(&self) -> &str;
    fn downloads(&self) -> u64;
    fn homepage(&self) -> &str;
    fn license(&self) -> &str;
    fn max_version(&self) -> &str;
    fn name(&self) -> &str;
    fn repository(&self) -> &str;
    fn keywords(&self) -> Vec<&str>;
    fn versions(&self) -> &[Version];
    fn created_at(&self) -> TimeStamp;
    fn updated_at(&self) -> TimeStamp;
    fn show_keywords(&self) -> String;
}

#[derive(Debug)]
pub(super) struct TimeStamp(DateTime<Utc>);

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.pad(&format!("{}", HumanTime::from(self.0)))
        } else {
            f.pad(&format!("{}", self.0.naive_local()))
        }
    }
}

impl CrateResponseExt for CrateResponse {
    fn description(&self) -> &str {
        self.crate_data.description.as_deref().unwrap_or_default()
    }

    fn documentation(&self) -> &str {
        self.crate_data.documentation.as_deref().unwrap_or_default()
    }

    fn downloads(&self) -> u64 {
        self.crate_data.downloads
    }

    fn homepage(&self) -> &str {
        self.crate_data.homepage.as_deref().unwrap_or_default()
    }

    fn license(&self) -> &str {
        self.crate_data.license.as_deref().unwrap_or_default()
    }

    fn max_version(&self) -> &str {
        &self.crate_data.max_version
    }

    fn name(&self) -> &str {
        &self.crate_data.name
    }

    fn repository(&self) -> &str {
        self.crate_data.repository.as_deref().unwrap_or_default()
    }

    fn keywords(&self) -> Vec<&str> {
        self.crate_data
            .keywords
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|s| s.as_str())
            .collect()
    }

    fn versions(&self) -> &[Version] {
        &self.versions
    }

    fn created_at(&self) -> TimeStamp {
        TimeStamp(self.crate_data.created_at)
    }

    fn updated_at(&self) -> TimeStamp {
        TimeStamp(self.crate_data.updated_at)
    }

    fn show_keywords(&self) -> String {
        self.crate_data
            .keywords
            .as_deref()
            .unwrap_or_default()
            .join(", ")
    }
}

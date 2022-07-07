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
    fn show_features(&self, verbose: bool) -> String;
    fn max_version_detailed(&self) -> Option<&Version>;
}

#[derive(Debug)]
pub(super) struct TimeStamp(DateTime<Utc>);

impl TimeStamp {
    const FORMAT: &'static str = "%c";
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = fmtools::format!(
            {self.0.naive_local().format(Self::FORMAT)}
            if f.alternate() {
                "    (" {HumanTime::from(self.0)} ")"
            }
        );
        f.pad(&text)
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
        self.crate_data
            .license
            .as_deref()
            .or_else(|| {
                self.max_version_detailed()
                    .and_then(|v| v.license.as_deref())
            })
            .unwrap_or_default()
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
        fmtools::join(", ", self.keywords()).to_string()
    }

    fn show_features(&self, verbose: bool) -> String {
        fmtools::format!(if let Some(features) =
            self.versions.first().map(|version| &version.features)
        {
            if verbose {
                let features = features.iter().map(|(feature, deps)| {
                    fmtools::format!(
                        {feature} ": " {fmtools::join(", ", deps)}
                    )
                });
                {
                    fmtools::join("\n", features)
                }
            } else {
                {
                    fmtools::join(", ", features.keys())
                }
            }
        })
    }

    fn max_version_detailed(&self) -> Option<&Version> {
        self.versions
            .iter()
            .find(|v| v.num == self.crate_data.max_version)
    }
}

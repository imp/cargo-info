#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use std::time::Duration;

use chrono_humanize::HumanTime;
use clap::{Parser, Subcommand};
use crates_io_api::{CrateResponse, SyncClient, Version};

use crates::CrateResponseExt;

mod crates;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.info {
        Info::Info { report } => report.report(),
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    info: Info,
}

#[derive(Debug, Subcommand)]
enum Info {
    Info {
        #[clap(flatten)]
        report: Report,
    },
}

#[derive(Debug, Parser)]
struct Report {
    #[clap(long, short)]
    /// Report documentation URL
    documentation: bool,
    #[clap(long, short = 'D')]
    /// Report number of crate downloads
    downloads: bool,
    #[clap(long, short = 'H')]
    /// Report crate homepage URL
    homepage: bool,
    #[clap(long, short)]
    /// Report crate repository URL
    repository: bool,
    #[clap(long, short)]
    /// Report more details
    verbose: bool,
    #[clap(long, short = 'V', action = clap::ArgAction::Count)]
    /// Report version history of the crate (5 last versions), twice for full history
    versions: u8,
    #[clap(long, short)]
    /// Report crate features
    features: bool,
    #[clap(name = "crate", required = true)]
    /// crates to report
    crates: Vec<String>,
}

impl Report {
    fn report(&self) -> anyhow::Result<()> {
        let client = SyncClient::new(
            "cargo-info (cargo-info@mountall.com)",
            Duration::from_millis(10),
        )?;
        self.crates
            .iter()
            .map(|krate| client.get_crate(krate))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .for_each(|krate| self.report_crate(krate));

        Ok(())
    }

    fn report_crate(&self, krate: CrateResponse) {
        let mut default = true;
        if self.documentation {
            default = false;
            fmtools::println!({ krate.documentation() });
        }

        if self.downloads {
            default = false;
            fmtools::println!({ krate.downloads() });
        }

        if self.homepage {
            default = false;
            fmtools::println!({ krate.homepage() });
        }

        if self.repository {
            default = false;
            fmtools::println!({ krate.repository() });
        }

        if self.versions > 0 {
            default = false;
            fmtools::println!({
                print_last_versions(krate.versions(), self.version_limit(), None, self.verbose)
            });
        }

        if self.features {
            default = false;
            fmtools::println!({ krate.show_features(self.verbose) });
        }

        if default {
            show_crate(krate, 5, self.verbose);
        }

        println!();
    }

    fn version_limit(&self) -> usize {
        match self.versions {
            0 => 0,
            1 => 5,
            _ => usize::MAX,
        }
    }
}

fn show_crate(krate: CrateResponse, limit: usize, verbose: bool) {
    // let width = 16;
    fmtools::println!(
        if verbose {
            {"Crate:":<16}{krate.name()} "\n"
            {"Version:":<16}{krate.max_version()} "\n"
            {"Description:":<16}{krate.description()} "\n"
            {"Downloads:":<16}{krate.downloads()} "\n"
            {"Homepage:":<16}{krate.homepage()} "\n"
            {"Documentation:":<16}{krate.documentation()} "\n"
            {"Repository:":<16}{krate.repository()} "\n"
            {"License:":<16}{krate.license()} "\n"
            {"Features:":<16}{krate.show_features(false)} "\n"
            {"Keywords:":<16}{krate.show_keywords()} "\n"
            {"Created:":<16}{krate.created_at():#} "\n"
            {"Updated:":<16}{krate.updated_at():#}
        } else {
            {"Crate:":<16}{krate.name()} "\n"
            {"Version:":<16}{krate.max_version()} "\n"
            {"Description:":<16}{krate.description()} "\n"
            {"Downloads:":<16}{krate.downloads()} "\n"
            {"Homepage:":<16}{krate.homepage()} "\n"
            {"Documentation:":<16}{krate.documentation()} "\n"
            {"Repository:":<16}{krate.repository()} "\n"
            {"Updated:":<16}{krate.updated_at():#} "\n"
            {"Version history:":<16} "\n\n"
            {print_last_versions(krate.versions(), limit, "  ", false)}
        }
    )
}

fn print_last_versions<'a>(
    versions: &[Version],
    limit: usize,
    prefix: impl Into<Option<&'a str>>,
    _verbose: bool,
) -> String {
    let prefix = prefix.into().unwrap_or_default();
    let version_width = versions
        .iter()
        .take(limit)
        .map(|v| v.num.len())
        .max()
        .unwrap_or(0);
    // Make sure the column header is taken into account and add a gutter.
    let version_width = std::cmp::max(version_width, "VERSION".len()) + 2;
    fmtools::format!(
        {prefix}{"VERSION",version_width:<1$}{"RELEASED":<16}{"DOWNLOADS":<11}"\n\n"
        for version in versions.iter().take(limit) {
            let created = HumanTime::from(version.created_at);
            {prefix}{version.num,version_width:<1$}{created:<16}{version.downloads:<11}
            if version.yanked {
                "\t\t(yanked)"
            }
            "\n"
        }
        let length = versions.len();
        if limit < length {
            "\n"
            {prefix}
            "... use -VV to show all "{length}" versions"
        }
    )

    // Consider adding some more useful information in verbose mode
}

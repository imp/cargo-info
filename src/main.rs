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
    #[clap(long, short = 'V', parse(from_occurrences))]
    /// Report version history of the crate (5 last versions), twice for full history
    versions: usize,
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
            println!("{}", krate.documentation());
        }

        if self.downloads {
            default = false;
            println!("{}", krate.downloads());
        }

        if self.homepage {
            default = false;
            println!("{}", krate.homepage());
        }

        if self.repository {
            default = false;
            println!("{}", krate.repository());
        }

        if self.versions > 0 {
            default = false;
            println!(
                "{}",
                print_last_versions(krate.versions(), self.version_limit(), self.verbose)
            );
        }

        if default {
            display_crate(krate, 5, self.verbose);
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

fn display_crate(krate: CrateResponse, limit: usize, verbose: bool) {
    let width = 16;
    let name = format!("{:<width$}{}", "Crate:", krate.name());
    let version = format!("{:<width$}{}", "Version:", krate.max_version());
    let description = format!("{:<width$}{}", "Description:", krate.description());
    let downloads = format!("{:<16}{}", "Downloads:", krate.downloads());
    let homepage = format!("{:<width$}{}", "Homepage:", krate.homepage());
    let documentation = format!("{:<width$}{}", "Documentation:", krate.documentation());
    let repository = format!("{:<width$}{}", "Repository:", krate.repository());
    let license = format!("{:<width$}{}", "License:", krate.license());
    let created = krate.created_at();
    let created_at = format!("{:<width$}{} ({:#})", "Created:", created, created);
    let updated = krate.updated_at();
    let updated_at = format!("{:<width$}{} ({:#})", "Updated:", updated, updated);
    let keywords = format!("{:<width$}{:?}", "Keywords:", krate.keywords());
    if verbose {
        println!("{name}\n{version}\n{description}\n{downloads}\n{homepage}\n{documentation}\n{repository}\n{license}\n{keywords}\n{created_at}\n{updated_at}");
    } else {
        let mut text = String::new();
        for line in print_last_versions(krate.versions(), limit, false).lines() {
            text += "\n";
            if !line.is_empty() {
                text = text + "  " + line;
            }
        }
        println!(
            "{name}\n{version}\n{description}\n{downloads}\n{homepage}\n{documentation}\n{repository}\n{updated_at}\n{}",
            format_args!("{:<16}\n{}", "Version history:", text)
        )
    }
}

fn print_last_versions(versions: &[Version], limit: usize, verbose: bool) -> String {
    let text = format!("{:<11}{:<#16}{:<11}\n", "VERSION", "RELEASED", "DOWNLOADS");

    if verbose {
        // Consider adding some more useful information in verbose mode
    }

    let text = versions.iter().take(limit).fold(text, |text, version| {
        format!("{text}\n{}", print_version(version, verbose))
    });

    let length = versions.len();
    if limit < length {
        format!("{text}\n\n... use -VV to show all {length} versions\n")
    } else {
        text
    }
}

fn print_version(version: &Version, _verbose: bool) -> String {
    let created = HumanTime::from(version.created_at);
    let yanked = if version.yanked { "\t\t(yanked)" } else { "" };
    format!(
        "{:<11}{:<16}{:<11}{yanked}",
        version.num, created, version.downloads
    )

    // Consider adding some more useful information in verbose mode
}

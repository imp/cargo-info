#[macro_use]
extern crate clap;
extern crate chrono;
extern crate chrono_humanize;
#[macro_use]
extern crate error_chain;
extern crate libcratesio;
extern crate pager;

use std::fmt;

use clap::{App, SubCommand, Arg, AppSettings, ArgMatches};
use pager::Pager;
use libcratesio::{CratesIO, Crate};

use errors::*;
use crates::PrintCrateInfo;

mod crates;
mod errors;

const CARGO: &str = "cargo";

#[derive(Clone, Debug, PartialEq)]
enum Flag {
    Repository,
    Documentation,
    Downloads,
    Homepage,
    Default,
}

#[derive(Clone, Debug)]
struct Report {
    flags: Vec<Flag>,
    verbose: bool,
    json: bool,
    versions: usize,
    keywords: bool,
}

impl Report {
    pub fn new(info: &ArgMatches) -> Self {

        let mut flags: Vec<Flag> = vec![];
        if info.is_present("repository") {
            flags.push(Flag::Repository);
        }
        if info.is_present("documentation") {
            flags.push(Flag::Documentation);
        }
        if info.is_present("downloads") {
            flags.push(Flag::Downloads);
        }
        if info.is_present("homepage") {
            flags.push(Flag::Homepage);
        }

        if flags.is_empty() {
            flags.push(Flag::Default);
        }

        let versions = match info.occurrences_of("versions") {
            0 => 0, // No flags - nothing to do
            1 => 5, // Single -V - show 5 last versions
            _ => usize::max_value(), // All the other cases - show everything
        };

        Report {
            flags: flags,
            verbose: info.is_present("verbose"),
            json: info.is_present("json"),
            versions: versions,
            keywords: info.is_present("keywords"),
        }
    }

    pub fn report(&self, name: &str) -> Result<String> {
        let response = CratesIO::query(name)
            .chain_err(|| "crates.io query failed")?;
        let mut output = String::new();

        if self.json {
            output = output + &self.report_json(&response);
        } else if let Ok(krate) = Crate::from_apiresponse(&response.as_data()?) {
            if self.versions > 0 {
                output = output + &self.report_versions(&krate, self.versions);
            } else if self.keywords {
                output = output + &self.report_keywords(&krate);
            } else {
                output = output + &self.report_crate(&krate);
            }
        };
        Ok(output)
    }

    pub fn report_json(&self, response: &CratesIO) -> String {
        if self.verbose {
            match response.as_json() {
                Ok(json) => format!("{:#}", json),
                _ => String::new(),
            }
        } else {
            response.raw_data().to_owned()
        }
    }

    pub fn report_crate(&self, krate: &Crate) -> String {
        let mut output = String::new();
        for flag in &self.flags {
            output = output +
                     &match *flag {
                         Flag::Repository => krate.print_repository(self.verbose),
                         Flag::Documentation => krate.print_documentation(self.verbose),
                         Flag::Downloads => krate.print_downloads(self.verbose),
                         Flag::Homepage => krate.print_homepage(self.verbose),
                         Flag::Default => krate.print_default(self.verbose),
                     }
        }
        output
    }

    pub fn report_versions(&self, krate: &Crate, limit: usize) -> String {
        if limit > 0 {
            krate.print_last_versions(limit, self.verbose)
        } else {
            String::new()
        }
    }

    pub fn report_keywords(&self, krate: &Crate) -> String {
        krate.print_keywords(self.verbose)
    }
}

// fn debug<T>(item: &T)
//     where T: fmt::Debug
// {
//     println!("{:#?}", item);
// }

fn print_report<T>(r: Result<T>)
    where T: fmt::Display
{
    match r {
        Ok(text) => println!("\n{}\n", text),
        Err(err) => println!("\n{}\n", err),
    }
}

fn main() {

    Pager::new().setup();

    let matches = App::new(CARGO)
        .bin_name(CARGO)
        .author(crate_authors!())
        .version(crate_version!())
        .about("Query crates.io registry for crates details")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("info")
            .setting(AppSettings::ArgRequiredElseHelp)
            .setting(AppSettings::TrailingVarArg)
            .arg(Arg::with_name("documentation")
                .short("d")
                .long("documentation")
                .help("Report documentation URL"))
            .arg(Arg::with_name("downloads")
                .short("D")
                .long("downloads")
                .help("Report number of crate downloads"))
            .arg(Arg::with_name("homepage")
                .short("H")
                .long("homepage")
                .help("Report home page URL"))
            .arg(Arg::with_name("repository")
                .short("r")
                .long("repository")
                .help("Report crate repository URL"))
            .arg(Arg::with_name("updated")
                .short("u")
                .long("updated")
                .help("Report new and recently updates crates")
                .conflicts_with_all(&["documentation",
                                      "downloads",
                                      "homepage",
                                      "repository",
                                      "json"]))
            .arg(Arg::with_name("json")
                .short("j")
                .long("json")
                .help("Report raw JSON data from crates.io")
                .conflicts_with_all(&["documentation",
                                      "downloads",
                                      "homepage",
                                      "repository",
                                      "updated"]))
            .arg(Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Report more details"))
            .arg(Arg::with_name("versions")
                .short("V")
                .long("versions")
                .multiple(true)
                .help("Report version history of the crate (5 last versions), twice for full \
                       history"))
            .arg_from_usage("<crate>... 'crate to query'"))
        .get_matches();

    if let Some(info) = matches.subcommand_matches("info") {
        if let Some(crates) = info.values_of("crate") {
            let rep = Report::new(info);
            for krate in crates {
                // debug(&krate);
                print_report(rep.report(krate));
            }
        }
    }
}

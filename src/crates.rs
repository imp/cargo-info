use std::fmt;

use chrono::{DateTime, UTC};
use chrono_humanize::HumanTime;
use libcratesio::{Crate, Version};

struct TimeStamp(DateTime<UTC>);

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.pad(&format!("{}", HumanTime::from(self.0)))
        } else {
            f.pad(&format!("{}", self.0.naive_local()))
        }
    }
}

struct TextOption<'a>(&'a Option<String>);

impl<'a> fmt::Display for TextOption<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let empty = String::new();
        let text = self.0.as_ref().unwrap_or(&empty);
        f.pad(text)
    }
}

fn print_version_header(verbose: bool) -> String {
    let output = format!("{:<11}{:<#16}{:<11}\n", "VERSION", "RELEASED", "DOWNLOADS");
    if verbose {
        // Consider adding some more useful information in verbose mode
        output + "\n"
    } else {
        output + "\n"
    }
}

fn print_version(v: &Version, verbose: bool) -> String {
    let created_at = TimeStamp(v.created_at);
    let mut output = format!("{:<11}{:<#16}{:<11}", v.num, created_at, v.downloads);

    if v.yanked {
        output = output + "\t\t(yanked)";
    }

    if verbose {
        // Consider adding some more useful information in verbose mode
        output + "\n"
    } else {
        output + "\n"
    }
}

pub trait PrintCrateInfo {
    fn print_repository(&self, verbose: bool) -> String;
    fn print_documentation(&self, verbose: bool) -> String;
    fn print_downloads(&self, verbose: bool) -> String;
    fn print_homepage(&self, verbose: bool) -> String;
    fn print_last_versions(&self, limit: usize, verbose: bool) -> String;
    fn print_keywords(&self, verbose: bool) -> String;
    fn print_default(&self, verbose: bool) -> String;
}

impl PrintCrateInfo for Crate {
    fn print_repository(&self, verbose: bool) -> String {
        let repository = TextOption(&self.repository);
        if verbose {
            format!("{:<16}{}", "Repository:", repository)
        } else {
            format!("{}", repository)
        }
    }

    fn print_documentation(&self, verbose: bool) -> String {
        let documentation = TextOption(&self.documentation);
        if verbose {
            format!("{:<16}{}", "Documentation:", documentation)
        } else {
            format!("{}", documentation)
        }
    }

    fn print_downloads(&self, verbose: bool) -> String {
        if verbose {
            format!("{:<16}{}", "Downloads:", self.downloads)
        } else {
            format!("{}", self.downloads)
        }
    }

    fn print_homepage(&self, verbose: bool) -> String {
        let homepage = TextOption(&self.homepage);
        if verbose {
            format!("{:<16}{}", "Homepage:", homepage)
        } else {
            format!("{}", homepage)
        }
    }

    fn print_last_versions(&self, limit: usize, verbose: bool) -> String {
        let mut output = print_version_header(verbose);
        for version in self.versions.iter().take(limit) {
            output += &print_version(version, verbose);
        }
        let length = self.versions.len();
        if limit < length {
            output += &format!("\n... use -VV to show all {} versions\n", length);
        }
        output
    }

    fn print_keywords(&self, verbose: bool) -> String {
        let keywords = "";
        if verbose {
            format!("{:#}", keywords)
        } else {
            format!("{}", keywords)
        }
    }

    fn print_default(&self, verbose: bool) -> String {
        let created_at = TimeStamp(self.created_at);
        let updated_at = TimeStamp(self.updated_at);

        let keywords = String::new();
            // self.krate["keywords"].members().filter_map(|jv| jv.as_str()).collect::<Vec<_>>();
        let description = TextOption(&self.description);
        let homepage = TextOption(&self.homepage);
        let documentation = TextOption(&self.documentation);
        let repository = TextOption(&self.repository);
        let license = TextOption(&self.license);

        if verbose {
            format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                   format_args!("{:<16}{}", "Crate:", self.name),
                   format_args!("{:<16}{}", "Version:", self.max_version),
                   format_args!("{:<16}{}", "Description:", description),
                   format_args!("{:<16}{}", "Downloads:", self.downloads),
                   format_args!("{:<16}{}", "Homepage:", homepage),
                   format_args!("{:<16}{}", "Documentation:", documentation),
                   format_args!("{:<16}{}", "Repository:", repository),
                   format_args!("{:<16}{}", "License:", license),
                   format_args!("{:<16}{:?}", "Keywords:", keywords),
                   format_args!("{:<16}{}  ({:#})", "Created at:", created_at, created_at),
                   format_args!("{:<16}{}  ({:#})", "Updated at:", updated_at, updated_at))
        } else {
            let mut versions = String::new();
            for line in self.print_last_versions(5, false).lines() {
                versions = versions + "\n";
                if !line.is_empty() {
                    versions = versions + "  " + line;
                }
            }

            format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                   format_args!("{:<16}{}", "Crate:", self.name),
                   format_args!("{:<16}{}", "Version:", self.max_version),
                   format_args!("{:<16}{}", "Description:", description),
                   format_args!("{:<16}{}", "Downloads:", self.downloads),
                   format_args!("{:<16}{}", "Homepage:", homepage),
                   format_args!("{:<16}{}", "Documentation:", documentation),
                   format_args!("{:<16}{}", "Repository:", repository),
                   format_args!("{:<16}{:#}", "Last updated:", updated_at),
                   format_args!("{:<16}\n{}", "Version history:", versions))
        }
    }
}

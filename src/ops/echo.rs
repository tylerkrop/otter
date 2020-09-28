use super::Operation;
use crate::constants::subcommands::echo::{args, ABOUT, NAME};
use crate::AppSettings;
use clap;

pub struct Echo {}

impl Operation for Echo {
    fn get_subcommand(&self) -> clap::App<'static, 'static> {
        clap::App::new(NAME).about(ABOUT).arg(
            clap::Arg::with_name(args::values::NAME)
                .min_values(0)
                .help(args::values::HELP),
        )
    }
    fn execute(&self, _settings: &mut AppSettings, matches: &clap::ArgMatches) {
        if let Some(values) = matches.values_of(args::values::NAME) {
            println!("{}", values.collect::<Vec<&str>>().join(" "));
        }
    }
}

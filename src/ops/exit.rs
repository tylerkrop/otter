use super::Operation;
use crate::constants::subcommands::exit::{ABOUT, NAME};
use crate::AppSettings;
use clap;

pub struct Exit {}

impl Operation for Exit {
    fn get_subcommand(&self) -> clap::App<'static, 'static> {
        clap::App::new(NAME).about(ABOUT)
    }
    fn execute(&self, settings: &mut AppSettings, _matches: &clap::ArgMatches) {
        settings.interactive = false;
        println!("Goodbye! 👋");
    }
}

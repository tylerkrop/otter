use super::Operation;
use crate::constants::subcommands::exit::{ABOUT, NAME};
use crate::AppSettings;
use clap;

pub struct Exit {}

impl Exit {
    pub fn new() -> Exit {
        Exit {}
    }
}

impl Operation for Exit {
    fn get_name(&self) -> &'static str {
        NAME
    }
    fn get_subcommand(&self) -> clap::App<'static, 'static> {
        clap::App::new(NAME).about(ABOUT)
    }
    fn execute(&self, settings: &mut AppSettings, _matches: &clap::ArgMatches) {
        settings.interactive = false;
        println!("Goodbye! 👋");
    }
}

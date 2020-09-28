use super::Operation;
use crate::constants::subcommands::ping::{ABOUT, NAME};
use crate::AppSettings;
use clap;

pub struct Ping {}

impl Operation for Ping {
    fn get_subcommand(&self) -> clap::App<'static, 'static> {
        clap::App::new(NAME).about(ABOUT)
    }
    fn execute(&self, _settings: &mut AppSettings, _matches: &clap::ArgMatches) {
        println!("Pong! 🏓");
    }
}

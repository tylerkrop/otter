mod echo;
mod exit;
mod ping;
use crate::AppSettings;
use clap;
pub use echo::Echo;
pub use exit::Exit;
pub use ping::Ping;
use std::collections::HashMap;

pub trait Operation {
    fn get_subcommand(&self) -> clap::App<'static, 'static>;
    fn execute(&self, app: &mut AppSettings, matches: &clap::ArgMatches);
}

pub struct OperationService {
    map: HashMap<String, Box<dyn Operation>>,
}

impl OperationService {
    pub fn new() -> OperationService {
        OperationService {
            map: HashMap::new(),
        }
    }

    pub fn register_operation(&mut self, operation: Box<dyn Operation>) {
        match self
            .map
            .insert(operation.get_subcommand().get_name().to_owned(), operation)
        {
            None => (),
            Some(op) => panic!(
                "Attempt to overwrite operation: {}",
                op.get_subcommand().get_name(),
            ),
        }
    }

    pub fn get_app(&self, clap_app: clap::App<'static, 'static>) -> clap::App<'static, 'static> {
        let mut clap_app = clap_app;
        for operation in self.map.values() {
            clap_app = clap_app.subcommand(operation.get_subcommand());
        }
        return clap_app;
    }

    pub fn execute(&self, settings: &mut AppSettings, subcommand: &Box<clap::SubCommand>) {
        self.map
            .get(&subcommand.name)
            .unwrap() // we can unwrap here because clap consumes invalid commands
            .execute(settings, &subcommand.matches)
    }
}

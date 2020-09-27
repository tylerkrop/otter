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
    fn get_name(&self) -> &'static str;
    fn get_subcommand(&self) -> clap::App<'static, 'static>;
    fn execute(&self, app: &mut AppSettings, matches: &clap::ArgMatches);
}

pub struct OperationService {
    map: HashMap<&'static str, Box<dyn Operation>>,
}

impl OperationService {
    pub fn new() -> OperationService {
        OperationService {
            map: HashMap::new(),
        }
    }

    pub fn register_operation(&mut self, operation: Box<dyn Operation>) {
        match self.map.insert(operation.get_name(), operation) {
            None => (),
            Some(op) => panic!("Attempt to overwrite operation: {}", op.get_name()),
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
            .get(subcommand.name.as_str())
            .unwrap()
            .execute(settings, &subcommand.matches)
    }
}

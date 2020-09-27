mod constants;
mod ops;
use clap;
use constants::app::{self, args};
use std::env;
use std::io::{self, Write};

pub struct AppSettings {
    interactive: bool,
}

pub struct App {
    app: clap::App<'static, 'static>,
    settings: AppSettings,
    operation_service: ops::OperationService,
}

impl App {
    pub fn new() -> App {
        let mut operation_service = ops::OperationService::new();
        operation_service.register_operation(Box::new(ops::Ping::new()));
        operation_service.register_operation(Box::new(ops::Exit::new()));
        operation_service.register_operation(Box::new(ops::Echo::new()));
        App {
            app: operation_service.get_app(
                clap::App::new(app::NAME)
                    .author(app::AUTHOR)
                    .version(app::VERSION)
                    .arg(
                        clap::Arg::with_name(args::interative::NAME)
                            .long(args::interative::NAME)
                            .short(args::interative::SHORT)
                            .help(args::interative::HELP),
                    ),
            ),
            settings: AppSettings { interactive: false },
            operation_service: operation_service,
        }
    }

    pub fn run(&mut self) {
        match self
            .app
            .get_matches_from_safe_borrow(env::args().collect::<Vec<String>>())
        {
            Ok(matches) => {
                self.handle_app_matches(&matches);
                if let Some(subcommand) = &matches.subcommand {
                    self.operation_service
                        .execute(&mut self.settings, subcommand);
                } else {
                    self.settings.interactive = true;
                }
                if self.settings.interactive {
                    self.run_interactive();
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }

    fn handle_app_matches(&mut self, matches: &clap::ArgMatches) {
        if matches.is_present("interactive") {
            self.settings.interactive = true;
        }
    }

    fn run_interactive(&mut self) {
        println!("{}", app::ASCII_NAME);
        while self.settings.interactive {
            let input = self.get_input();
            match self.app.get_matches_from_safe_borrow(input) {
                Ok(matches) => {
                    self.handle_app_matches(&matches);
                    if let Some(subcommand) = &matches.subcommand {
                        self.operation_service
                            .execute(&mut self.settings, subcommand);
                    } else {
                        println!("flags updated");
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }

    fn get_input(&self) -> Vec<String> {
        let mut input = String::new();
        print!("{}> ", app::NAME);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<String> = [
            vec![String::from(app::NAME)],
            input.split(" ").map(|x| x.trim().to_string()).collect(),
        ]
        .concat();
        return args;
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}

mod constants;
mod ops;
use clap;
use colored::*;
use constants::{
    app::{self, args},
    symbols,
};
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
        operation_service.register_operation(Box::new(ops::Ping {}));
        operation_service.register_operation(Box::new(ops::Exit {}));
        operation_service.register_operation(Box::new(ops::Echo {}));
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
                    )
                    .arg(
                        clap::Arg::with_name(args::color::NAME)
                            .long(args::color::NAME)
                            .takes_value(true)
                            .possible_value(args::color::AUTO)
                            .possible_value(args::color::ALWAYS)
                            .possible_value(args::color::NEVER),
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
        if matches.is_present(args::interative::NAME) {
            self.settings.interactive = true;
        }
        if let Some(value) = matches.value_of(args::color::NAME) {
            match value {
                args::color::ALWAYS => control::set_override(true),
                args::color::NEVER => control::set_override(false),
                _ => control::unset_override(),
            }
        }
    }

    fn run_interactive(&mut self) {
        println!("{}", app::ASCII_NAME.red().bold());
        while self.settings.interactive {
            let input = self.get_input();
            match self.app.get_matches_from_safe_borrow(input) {
                Ok(matches) => {
                    self.handle_app_matches(&matches);
                    if let Some(subcommand) = &matches.subcommand {
                        self.operation_service
                            .execute(&mut self.settings, subcommand);
                    } else {
                        println!("{} {}", symbols::CHECK.green().bold(), args::ARGS_UPDATED);
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }

    fn get_input(&self) -> Vec<String> {
        let mut input = String::new();
        print!("{} {} ", app::NAME.green().bold(), "$".yellow().bold());
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

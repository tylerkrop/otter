pub mod app {
    pub const NAME: &str = "otter";
    pub const AUTHOR: &str = "Tyler Kropiewnicki <tylerkrop@gmail.com>";
    pub const VERSION: &str = "0.1.0";
    pub const ASCII_NAME: &str = "         __  __\n  ____  / /_/ /____  _____\n / __ \\/ __/ __/ _ \\/ ___/\n/ /_/ / /_/ /_/  __/ /\n\\____/\\__/\\__/\\___/_/";
    pub mod args {
        pub mod interative {
            pub const NAME: &str = "interactive";
            pub const SHORT: &str = "i";
            pub const HELP: &str = "Runs otter in interactive mode";
        }
        pub const ARGS_UPDATED: &str = "✔ Arguments updated!";
    }
}
pub mod subcommands {
    pub mod echo {
        pub const NAME: &str = "echo";
        pub const ABOUT: &str = "Echos values back to the console";
        pub mod args {
            pub mod values {
                pub const NAME: &str = "values";
                pub const HELP: &str = "Values to print to console";
            }
        }
    }
    pub mod exit {
        pub const ABOUT: &str = "Exits program";
        pub const NAME: &str = "exit";
    }
    pub mod ping {
        pub const NAME: &str = "ping";
        pub const ABOUT: &str = "Responds with pong";
    }
}

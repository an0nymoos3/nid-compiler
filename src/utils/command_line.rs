use std::env;

#[derive(Debug)]
pub struct Args {
    pub filename: String,
    pub debug: bool,
    pub help: bool,
}

pub fn build_args() -> Args {
    let mut args: Args = Args {
        filename: String::new(),
        debug: false,
        help: false,
    };

    let cmd_line: Vec<String> = env::args().collect();

    for arg in cmd_line.iter() {
        if arg.contains(".nid") {
            args.filename = arg.to_owned();
        }
        if arg == "--debug" || arg == "-d" {
            args.debug = true;
        }
        if arg == "--help" || arg == "-h" {
            args.help = true;
        }
    }

    args
}

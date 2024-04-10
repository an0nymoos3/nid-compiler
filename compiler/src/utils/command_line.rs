use std::env;

#[derive(Debug)]
/// Possible args that can be used when running compiler.
pub struct Args {
    pub filename: String,
    pub debug: bool,
    pub help: bool,
    pub ext_ins: bool,
}

/// Reads and returns the correct command line args passed by user.
pub fn build_args() -> Args {
    let mut args: Args = Args {
        filename: String::new(),
        debug: false,
        help: false,
        ext_ins: false,
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
        if arg == "--extended-instructions" || arg == "-e" {
            args.ext_ins = true;
        }
    }

    args
}

/// Prints this not so helpful help message.
pub fn print_help() {
    let mut message: String = String::new();
    message.push_str("nid-compiler [options] [target].nid\n");
    message.push_str("Options:\n");
    message.push_str("-h | --help                   Prints this message.\n");
    message.push_str("-d | --debug                  Prints useful debug info.\n");
    message.push_str("-e | --extended-instructions  Allows for extra hardware instructions, requires support for extra instructions, meant to be hardware accelerated.\n");

    println!("{}", message);
}

use clap::Parser;
use cxxd::{execute_command, Args};

fn main() {
    let args = Args::parse();
    let exit_code = execute_command(&args);
    std::process::exit(exit_code);
}

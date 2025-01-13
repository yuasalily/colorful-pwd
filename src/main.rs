mod cli;
mod pwd;

use clap::Parser;
use pwd::pwd;

fn main() {
    let args = cli::Args::try_parse().unwrap();

    let path = pwd(args.physical);

    println!("Hello {}!", path);
}

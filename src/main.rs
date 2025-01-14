mod cli;
mod colorize;
mod pwd;

use clap::Parser;
use colorize::colorize;
use pwd::pwd;

fn main() {
    let args = cli::Args::try_parse().unwrap();

    let path = pwd(args.physical);

    let colorful_path = colorize(args.colorful, path);

    println!("Hello {}!", colorful_path);
}

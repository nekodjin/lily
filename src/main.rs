use clap::Parser as _;

mod args;

mod stage1;

fn main() {
    let args = args::Args::parse();

    println!("File: {:?}", args.file);
}

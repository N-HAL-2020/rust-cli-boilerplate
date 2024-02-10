use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[clap(short, long)]
    verbose: bool,
}

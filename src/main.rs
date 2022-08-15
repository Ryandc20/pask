use clap::Parser;

mod cl;
mod task;
mod ui;

fn main()
{
    let args = cl::Cli::parse();
}

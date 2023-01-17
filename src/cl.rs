use clap::{Parser, Subcommand, Args, ValueEnum};
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Will read from Cargo.toml
pub struct Cli {
    #[arg(value_enum)]
    pub list: Lists,
    #[clap(subcommand)] 
    pub command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Lists {
    /// List that does not correspond to date
    Goals, 
    /// Daily list 
    Day, 
    /// Weekly list
    Week,
    /// Monthly list 
    Month,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a file to a todo list 
    Add(Add),
    /// Delete A task from a list.
    Delete(Delete), 
    /// Change a task from incomplete to Complete
    Complete(Complete),
    /// Display the lists of tasks 
    Display,
    /// Open an interactive interface
    Gui,
}

#[derive(Args)]
pub struct Add {
    /// Desc of task must be unique to the list 
    pub desc: String,  
    /// Start time of the task if left blank will have no start time
    pub start_time: Option<String>,
    /// End time of the task if left blank will have no start time
    pub end_time: Option<String>,
}

#[derive(Args)]
pub struct Delete {
    /// Desc of task to delete 
    pub desc: String,
}

#[derive(Args)]
pub struct Complete {
    /// Desc of task to 
    pub desc: String, }

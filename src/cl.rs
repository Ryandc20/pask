//! Handle the command line argument parsing 

use clap::{Parser, Args, Subcommand, ValueEnum};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli 
{
    /// What action to take
    #[clap(subcommand)]
    action: Action,

}

#[derive(Debug, Subcommand)]
enum Action 
{
    /// Add an item to the list 
    #[clap(arg_required_else_help = true)]
    Add {
        /// The title of the task.
        title: String,
        /// The description of the task 
        description: Option<String>,
        /// The type of tasks will default to Day 
        #[clap(subcommand)] 
        // Will treat None the same as Type::day 
        task_type: Option<Type>,
    }, 
    /// Remove an item to the list 
    #[clap(arg_required_else_help = true)]
    Remove {
        /// The title of the task to remove 
        title: String,
        #[clap(subcommand)]
        /// The type of tasks will default to day. 
        task_type: Option<Type>
    },
    /// Mark an action as completed. If already completed will do nothing. 
    #[clap(arg_required_else_help = true)]
    Completed,
    /// Mark an action as incomplete. If already incomplete will do nothing.
    #[clap(arg_required_else_help = true)]
    Incomplete,
    /// Print the ordered task list to the schedule 
    #[clap(arg_required_else_help = true)]
    Print,
    /// Print the task list in a schedule format 
    #[clap(arg_required_else_help = true)]
    Schedule,
    /// Open the GUI application 
    #[clap(arg_required_else_help = true)]
    Gui,
}


// Contains the type of tasks 
#[derive(Debug, Subcommand)]
enum Type 
{
    Day {
        /// Time in format hh:mm using 24 hour time. Default will be no time 
        time: Option<String>,
    },
    Weekly {
        /// Time in format hh:mm using 24 hour time. Default will be no time 
        time: Option<String>,
        // The date of the start of the week must be a sunday or action will fail.
        // Default will be current week. In format

    },
    Monthly {
        /// Time in format hh::mm using 24 hour time. Default will be no time 
        time: Option<String>,
        /// Default month will be current month
        month: Option<String>,
        /// Default year will be current year 
        year: Option<String>
    },
    Yearly {
        /// Time in format hh::mm using 24 hour time. Default will be no time 
        time: Option<String>,
        // Default year will be current year 
        year: Option<String>
    },
    /// Task with no due date or time.
    Inbox, 
}

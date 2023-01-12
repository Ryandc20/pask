use std::fs;
use std::env;

use clap::Parser;
use chrono::prelude::*;
use dirs::home_dir;

mod cl;
mod tasks;
mod tui;
use cl::*;
use tasks::*;

/// Gets the path to the current list to manage tasks for.
fn get_file_name(list_type: Lists) -> String {
    // Get date information 
    let date = Local::now().date_naive();
    let day = date.day().to_string();
    let month = date.month().to_string();
    let year = date.year().to_string();

    // Get the date for the first sunday of the current week 
    let week_day = date.week(Weekday::Sun).first_day().day().to_string();

    let postfix = ".json";

    // Get the name of the list
    let list_name = match list_type {
        Lists::Goals => "goals".to_owned(),
        Lists::Day => day + "-" + &month +  "-" + &year + "-day", 
        Lists::Week => week_day + "-" + &month + "-" + &year + "-week", 
        Lists::Month => month + "-month",
    };

    let file_name = list_name + postfix;

    // Return the full file path to the specific requested list
    file_name.clone()
}

fn main() {
    let args: Cli = cl::Cli::parse();

    // Change current working directory to home
    let home = home_dir().unwrap();
    assert!(env::set_current_dir(home).is_ok());
    
    // Create a directory if it does not exist 
    assert!(fs::create_dir_all(".pask").is_ok());

    assert!(env::set_current_dir(".pask").is_ok());
    // Get the current file name to load in the struct
    let file_name = get_file_name(args.list);

    let mut tasks = match Tasks::get_tasks(&file_name) {
        Ok(x) => x,
        Err(_) => {
            println!("The file read failed");
            return;
        },
    };

    match args.command {
        Commands::Add(x) => {
            // convert to task struct 
            let task = Task::from_add(x).unwrap();

            // Add to task file 
            tasks.add_task(task);

            tasks.write_tasks(&file_name);
        },
        Commands::Delete(x) => {
            // Delete the task
            tasks.del_task(x.desc);
            // Write the task back to the file 
            tasks.write_tasks(&file_name);
        },

        Commands::Complete(x) => {
            // Change the task to complete 
            tasks.complete_task(x.desc);
            // Write the tasks back to the file 
            tasks.write_tasks(&file_name);
        },
        Commands::Display => {
            println!("{}", tasks);
        },
        Commands::Gui => {
            println!("This is yet to be implemented");
            println!("{}", tasks);
        }
    };
}

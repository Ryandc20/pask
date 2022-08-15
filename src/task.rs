use super::cl;
use chrono::Local;
use serde::{Serialize, Deserialize};

/// Contains the defination for the task struct 
#[derive(Serialize, Deserialize, Debug)]
pub struct Task 
{
    name: String,               // The name of the task 
    description: String,        // The description of the task 
    start_time: (u8, u8),       // The start time of the task. (hour, minute) 
    end_time: (u8, u8),         // The end time of the task. (hour, minute) 
    task_type: Type,            // Type of task determines what file to write to 
}


// Type of task 
enum Type 
{
    Day,
    Week,
    Month,
    Inbox,  // For tasks that do not have a date
}


impl Task 
{
    pub fn new(args: cl::Cli) -> Self 
    {
        args.action;
    }
}

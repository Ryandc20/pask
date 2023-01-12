use std::{fmt, fs, path::Path};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};
use serde_json;
use crate::cl::Add;

/// Reresents a signle task item 
#[derive(Serialize, Deserialize)]
#[derive(Eq)]
pub struct Task { /// Description of the task
    desc: String,              
    /// Hour minute represents the time to start the task  
    start_time: Option<(u8, u8)>,
    /// Hour minute represents the time to end the task
    end_time: Option<(u8, u8)>,
    /// If the task has been completed or not 
    completed: bool,

}
    
impl Task {
    /// Takes in a add struct and returns the struct representing the task.
    pub fn from_add(add: Add) -> Result<Self, &'static str> {
        // Parse a string in to hours and minutes. While also making sure it is a valid string 
        let start_time: Option<(u8, u8)> = match add.start_time {
            Some(x) => {
                match string_to_time(x) {
                    Ok(x) => Some(x),
                    Err(x) => return Err(x),
                } 
            },
            None => None,
        };
        let end_time: Option<(u8, u8)> = match add.end_time {
            Some(x) => {
                match string_to_time(x) {
                    Ok(x) => Some(x), 
                    Err(x) => return Err(x),
                }
            },
            None => None,
        };
        Ok(Task { 
            desc: add.desc,
            start_time,
            end_time,
            completed: false,
        })
    }
}    

// Implement ordering for the task 
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        if self.start_time.is_some() && other.start_time.is_some() {
            self.start_time.unwrap().0 == other.start_time.unwrap().0 && 
                self.start_time.unwrap().1 == other.start_time.unwrap().1
        } else if self.start_time.is_some() {
            false
        } else if other.start_time.is_some() {
            false
        } else {
            true
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start_time.is_some() && other.start_time.is_some() {
            if self.start_time.unwrap().0 != other.start_time.unwrap().0 {
                self.start_time.unwrap().0.cmp(&other.start_time.unwrap().0)
            } else {
                self.start_time.unwrap().1.cmp(&other.start_time.unwrap().1)
            }
        } else if self.start_time.is_some() {
            Ordering::Less
        } else if other.start_time.is_some() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}


// Implement the ability to display tasks.
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_completed = if self.completed {
            "[x]"
        } else {
            "[ ]"
        };
        
        if self.start_time.is_some() && self.end_time.is_some() {
            write!(f,"{} {} {:02}:{:02} - {:02}:{:02}", is_completed, self.desc, self.start_time.unwrap().0, 
                self.start_time.unwrap().1, self.end_time.unwrap().0, self.end_time.unwrap().1)?;
        } else if self.start_time.is_some() {
            write!(f,"{} {} {:02}:{:02}", is_completed, self.desc, self.start_time.unwrap().0, 
                self.start_time.unwrap().1)?;
        } else {
            write!(f,"{} {}", is_completed, self.desc)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn get_tasks(file_name: &String) -> Result<Self, ()> {
        // Checks if the file exists 
        if Path::new(file_name).exists() {
            Self::read_tasks(file_name)
        } else {
            Ok(Self { tasks: vec![] })
        }
    }


    /// If write fails will return false
    pub fn write_tasks(&mut self, file_name: &String) -> bool {
        // Make sure the tasks are in sorted order by start time
        // Seralize struct into json 
        let file_content = match serde_json::to_string(self) {
            Ok(x) => x,
            Err(_) => return false,
        };

        fs::write(file_name, file_content).unwrap();

        return true;
    }
    
    /// Add a tasks to the set of tasks 
    pub fn add_task(&mut self, task: Task) {
        // make sure the task desc is not already contained within the list. If so append with a
        // number
        self.tasks.push(task); 
        
        // Make sure the list is still sorted by time 
        self.sort_tasks();
    }

    pub fn complete_task(&mut self, desc: String) {
        // Find task 
        for task in &mut self.tasks {

            if task.desc == desc {
                task.completed = true;
                return;
            }
        }
    }

    pub fn del_task(&mut self, desc: String) {
        let mut count = 0;

        let mut index: usize = 0;
        let mut remove = false;
        for task in &self.tasks {
            
            if task.desc == desc {
                index = count;
                remove = true;
            }
            count += 1;
        }
        if remove {
            self.tasks.remove(index);
        }
    }

    // Sorts the tasks by start time will be implemented soon 
    fn sort_tasks(&mut self) {
        self.tasks.sort();
    }
    
    /// Will return none if file does not exist or if a parsing error occurs.
    fn read_tasks(file_name: &String) -> Result<Self, ()> {
        // Get file content into a string 
        let file_content = match fs::read_to_string(file_name) {
            Ok(x) => x,
            Err(_)=> return Err(()),
        };
        
        // Deseralize content into task struct and reutrn 
        let tasks: Self = match serde_json::from_str(&file_content) {
            Ok(x) => x,
            Err(_) => return Err(()),
        };
        Ok(tasks)
    }
}

impl fmt::Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.tasks {
            writeln!(f, "{}", item)?;
        } 
        Ok(())
    }
}

/// String to time in format hour, minutes.
fn string_to_time(string: String) -> Result<(u8, u8), &'static str> {
    let str_split: Vec<&str> = string.split(":").collect();
    if str_split.len() != 2 {
        return Err("Input should be in format hour::minutes");
    }
    if str_split[0].len() != 1 && str_split[0].len() != 2 {
        return Err("Number of digits for hours is incorrect");
    }
    if str_split[1].len() != 1 && str_split[1].len() != 2 {
        return Err("Number of digits for minutes is incorrect");
    }

    let hour: u8 = match str_split[0].parse() {
        Ok(x) => x, 
        Err(_) => return Err("Hours does not contain all numbers"),
    };
    let minute: u8 = match str_split[1].parse() {
        Ok(x) => x,
        Err(_) => return Err("Minutes does not contain all numbers"),
    };
    
    Ok((hour, minute))
}

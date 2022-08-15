# A simple personal task manager for the command line  
## Todo
- [] Find the best way to store these tasks. Could use serilization to store tasks  

## Features
- Files tasks/ list 
    - todo tasks yet to be done 
    - completed 
    - Will send a reminder when completed 
    - Daily task file will have file name YYYY-MM-DD and be in data/daily.
    - Weekly taks will have file name and be in data/weekly 
    - Monthly tasks will have file name YYYY-MM
    - Inbox stores all task
- Task info
    - Name 
    - Time to start
    - Time to finish
    - Complete or not
- arguments (command line options)
    - 
- Other features 
    - Notifications for both the start and finish of the task 
    - The ability to display the task as a schedule 
    - Will have a configuration file probably use toml or something else If can not think of
      anything else will just use shell variables 
        - Allowing to disable push Notifications ex. 
    - Will use some sort of coloring to show if the task is overdue or not 


## Dependencies 
- CLAP (Command Line Arguments paser)
    - Will use in cl.rs to parse arguments given by the command line 
- chrone (Date and time)
    - Use for getting the data and time of tasks. 

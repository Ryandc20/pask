// A terminal user interface to allow user to manage tasks more efficently
use crate::Tasks;
use crate::Task;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

/// Represents the current mode the user is in
enum InputMode {
    Normal,
    Insert,
    Edit,   // Edit will define 
}

/// Holds the info for each task
struct AppInfo {
    // Current task that will be added 
    task: Task, 

    // Contains the list of tasks 
    tasks: Tasks,

    // Current mode 
    mode: InputMode,

    // Represents the current row of the cursor
    cursor_row: u16,
}

impl AppInfo {
    pub fn new(tasks: Tasks) -> Self {
        Self {
            task: Task::new(),
            tasks, 
            mode: InputMode::Normal,
            cursor_row: 0,
        }
    }
}


/// Will run the terminal gui on the given struct of tasks.
pub fn run_ui(tasks: Tasks) -> Result<Tasks, io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Creates a struct that represents the info the app needs 
    let mut app_info = AppInfo::new(tasks);

    // Runs the app 
    ui_loop(&mut terminal, &mut app_info).unwrap();

    // Close out of the app properly
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(app_info.tasks) 
}

/// The main loop for the ui 
fn ui_loop<T: Backend>(terminal: &mut Terminal<T>, app_info: &mut AppInfo) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, &app_info))?;

        if let Event::Key(key) = event::read()? {
            match app_info.mode {
                InputMode::Insert => match key.code {
                    KeyCode::Char(c) => {
                        app_info.task.desc.push(c);
                    },
                    KeyCode::Backspace => {
                        app_info.task.desc.pop();
                    },
                    KeyCode::Esc => {
                        app_info.mode = InputMode::Normal
                    }
                    KeyCode::Enter => {
                        app_info.tasks.add_task(app_info.task.clone());
                        app_info.task = Task::new();
                    }
                    _ => {},
                },
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app_info.mode = InputMode::Insert;
                    },
                    KeyCode::Char('q') => {
                        return Ok(());
                    },
                    KeyCode::Char('e') => {
                        app_info.mode = InputMode::Edit;
                    },

                    _ => {},
                },

                InputMode::Edit => match key.code {
                    KeyCode::Esc => {
                        app_info.mode = InputMode::Normal;
                        app_info.cursor_row = 0;
                    },
                    // Will delete the tasks hovered on 
                    KeyCode::Char('d') => {
                        if !app_info.tasks.tasks.is_empty() {
                            app_info.tasks.del_task_idx(app_info.cursor_row as usize);
                            if app_info.cursor_row > 0 {
                                app_info.cursor_row -= 1;
                            }
                        }
                    },
                    KeyCode::Enter => {
                        if !app_info.tasks.tasks.is_empty() {
                            app_info.tasks.complete_task_idx(app_info.cursor_row as usize);
                        }
                    },
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app_info.cursor_row > 0 {
                            app_info.cursor_row -= 1;
                        }
                    },
                    KeyCode::Down | KeyCode::Char('j') => {
                        if usize::from(app_info.cursor_row + 1) < app_info.tasks.tasks.len() {
                            app_info.cursor_row += 1;
                        }
                    },
                    _ => {},
                },
            }
        }
    }
}

/// creates the layout for the ui
fn draw_ui<T: Backend>(f: &mut Frame<T>, app_info: &AppInfo) {
    // Determines the layout for the app
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
                .as_ref(),
        )
        .split(f.size());

    // Determines the help bar based on what mode you are in
    let (msg, style) = match app_info.mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start inserting a task,"),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing tasks"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Insert => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, and start changing tasks"),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
        InputMode::Edit => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, and start changing tasks, "),
                Span::styled("J, K", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to move the cursor up and down(can also use arrow keys ), "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to complete / uncomplete the task you are hovering over "),
                Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to delete task"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    // Create the input box 
    let input = Paragraph::new(app_info.task.desc.as_ref())
        .style(match app_info.mode {
            InputMode::Insert => Style::default().fg(Color::LightBlue),
            _ => Style::default().fg(Color::Blue),
        })
        .block(Block::default().borders(Borders::ALL).title("Enter Task"));
    f.render_widget(input, chunks[1]);


    // Displays the task in a new chunk
    let mut tasks_disp: Vec<ListItem> = vec![];
    for task in &app_info.tasks.tasks {
        tasks_disp.push(ListItem::new(format!("{}", task)));
    }
    let tasks_list = List::new(tasks_disp)
        .block(Block::default().borders(Borders::ALL).title("Tasks"));
    f.render_widget(tasks_list, chunks[2]);

    // Turn on the cursor 
    match app_info.mode {
        InputMode::Insert => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app_info.task.desc.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        },

        InputMode::Edit => {
            f.set_cursor(
                chunks[2].x + 2,
                chunks[2].y + app_info.cursor_row + 1,
            )
        }

        _ => {},
    }
}

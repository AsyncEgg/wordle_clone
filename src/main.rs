use wordle_clone::{Wordle, GameState};

fn nm() {
    let mut wordle = Wordle::default();

    loop {
        match wordle.game_state {
            GameState::Guessing => {
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read input");
                let guess = String::from(guess.trim());
                wordle.submit_and_test_guess(guess);
            }
            GameState::Lost => {
                println!("You lose!; Correct word was: {}", wordle.word);
                break
            },
            GameState::Won => {
                println!("You win!");
                break
            },
        }

        println!("{:#?}", wordle.guesses_map)
    }
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

struct App {
    input: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {

                match key.code {

                    KeyCode::Enter => {                        
                        let drain: String = app.input.drain(..).collect();

                        if drain == String::from("quit") || drain == String::from("q") {
                            return Ok(());
                        }

                    }

                    KeyCode::Esc => {
                        return Ok(())
                    }

                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(3)
            ]
            .as_ref(),
        )
        .split(f.size());
    
    let vertical_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .margin(0)
    .constraints(
        [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]
        .as_ref(),
    )
    .split(chunks[0]);

    for x in 0..5 {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(vertical_chunks[x]);

        for x in 0..5 {
            let paragraph = Paragraph::new("hi")
            .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, horizontal_chunks[x]); 
        }

        
    }

    

    let input = Paragraph::new(&*app.input)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Primorial Number Generator"));
    f.render_widget(input, chunks[1]);
    f.set_cursor(
        chunks[1].x + app.input.width() as u16 + 1,
        chunks[1].y + 1,
    );

    //let instructions = String::from("type \"q\" or \"quit\" to quit | press ENTER to submit number | use arrow keys to scroll | type \"write\" to write data to file | type \"read\" to Read Data from file | type \"output\" to enable or disable output");

    //let instructions = string_wrap(&instructions, outer_chunks[1].width - 3, &OutputMode::Message);

    //let instructions = Paragraph::new(instructions)
    //    .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
    //    .block(Block::default().borders(Borders::ALL));

    //f.render_widget(instructions, outer_chunks[1]);
}

fn string_wrap(string: &String, chunk_width: u16) -> String {
    string
    .chars()
    .collect::<Vec<_>>()
    .chunks(chunk_width.into())
    .into_iter()
    .map(|chunk| chunk.into_iter().collect::<String>())
    .collect::<Vec<String>>()
    .join("\n")
}
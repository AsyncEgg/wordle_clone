use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use wordle_clone::{GameState, Wordle};

struct App {
    input: String,
    wordle: Wordle,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            wordle: Wordle::default(),
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
        match app.wordle.game_state {
            GameState::Guessing => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            let drain: String = app.input.drain(..).collect();
                            app.wordle.submit_and_test_guess(drain.to_ascii_lowercase());
                        }

                        KeyCode::Esc => return Ok(()),

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
            GameState::Lost => {
                //println!("You lose!; Correct word was: {}", app.wordle.word); //TODO implement popup here
                app.wordle.new_random_word();
                app.wordle.game_state = GameState::Guessing
            }
            GameState::Won => {
                //println!("You win!"); //TODO implement popup here
                app.wordle.new_random_word();
                app.wordle.game_state = GameState::Guessing
            }
        }

        terminal.draw(|f| ui(f, &app))?;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let number_of_letters: usize = 5;
    let number_of_guesses = 5;

    let outer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Max(5 * number_of_letters as u16),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Max(3 * number_of_letters as u16),
                Constraint::Length(1),
                Constraint::Max(3),
                Constraint::Percentage(0),
            ]
            .as_ref(),
        )
        .split(outer_chunks[1]);

    let mut vertical_constraints = gen_constraints(number_of_guesses, Constraint::Length(3));
    vertical_constraints.push(Constraint::Length(3));

    let board = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(vertical_constraints.as_ref())
        .split(chunks[0]);

    let mut horizontal_constraint = gen_constraints(number_of_letters, Constraint::Max(5));
    horizontal_constraint.push(Constraint::Percentage(0));

    for row in board.clone() {
        let column = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(horizontal_constraint.as_ref())
            .split(row);
        for col in column {
            f.render_widget(
                Paragraph::new("").block(Block::default().borders(Borders::ALL)),
                col,
            );
        }
    }

    let guesses = &app.wordle.guesses_map;

    for (row_index, (guess_index, (word, guess))) in
        (0..number_of_guesses).into_iter().zip(guesses.iter())
    {
        let column = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(horizontal_constraint.as_ref())
            .split(board[row_index]);

        for col in 0..number_of_letters {
            let vec_word = word.chars().map(|c| c.to_string()).collect::<Vec<String>>();

            match guess[col] {
                wordle_clone::LetterMatch::Belongs => f.render_widget(
                    Paragraph::new(vec_word[col].clone())
                        .style(Style::default().fg(Color::Green))
                        .block(Block::default().borders(Borders::ALL)),
                    column[col],
                ),
                wordle_clone::LetterMatch::NotInWord => f.render_widget(
                    Paragraph::new(vec_word[col].clone())
                        .block(Block::default().borders(Borders::ALL)),
                    column[col],
                ),
                wordle_clone::LetterMatch::BelongsElsewhere => f.render_widget(
                    Paragraph::new(vec_word[col].clone())
                        .style(Style::default().fg(Color::Yellow))
                        .block(Block::default().borders(Borders::ALL)),
                    column[col],
                ),
            }
        }
    }

    let input = Paragraph::new(&*app.input)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, chunks[2]);
    f.set_cursor(chunks[2].x + app.input.width() as u16 + 1, chunks[2].y + 1);
}

fn gen_constraints(n: usize, c: Constraint) -> Vec<Constraint> {
    vec![c; n]
}

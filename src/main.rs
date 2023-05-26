use core::time;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use wordle_clone::{GameState, Wordle};

struct App {
    input: String,
    wordle: Wordle,
    show_word_popup: bool,
    show_instructions_popup: bool,
    restart_keys: bool
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            wordle: Wordle::default(),
            show_word_popup: false,
            show_instructions_popup: true,
            restart_keys: false,
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
                app.restart_keys = false;
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
                        KeyCode::Left => app.show_word_popup = true,
                        _ => {}
                    }
                }
            }
            GameState::Lost => {
                app.show_word_popup = true;
                app.show_instructions_popup = false;
                terminal.draw(|f| ui(f, &app))?;
                
                thread::sleep(time::Duration::from_secs(5));
                app.restart_keys = true;
                app.wordle.new_random_word();
                app.wordle.game_state = GameState::Guessing;
                app.show_word_popup = false;
                app.wordle.bad_guess = Vec::new();
                app.wordle.good_guess = Vec::new();
                app.wordle.perfect_guess = Vec::new();

            }
            GameState::Won => {
                app.show_word_popup = true;
                app.show_instructions_popup = false;
                terminal.draw(|f| ui(f, &app))?;
                
                thread::sleep(time::Duration::from_secs(5));
                app.restart_keys = true;
                app.wordle.new_random_word();
                app.wordle.game_state = GameState::Guessing;
                app.show_word_popup = false;
                app.wordle.bad_guess = Vec::new();
                app.wordle.good_guess = Vec::new();
                app.wordle.perfect_guess = Vec::new();
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
                Constraint::Percentage(20),
                Constraint::Max(5 * number_of_letters as u16),
                Constraint::Percentage(20),
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
                Constraint::Length(5),
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

    let text = vec![
        string_to_styled_span(" qwertyuiop", app, app.restart_keys),
        string_to_styled_span(" asdfghjkl", app, app.restart_keys),
        string_to_styled_span(" zxcvbnm", app, app.restart_keys),
    ];
    let keys_pressed = 
    Paragraph::new(text).block(
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    );
    f.render_widget(keys_pressed, chunks[3]);
    
    

    if app.show_instructions_popup {
        let instructions =
            Paragraph::new("Type to guess 5 letter word\nEnter to submit / Continue\nEsc to exit");
        let area = centered_rect(60, 40, outer_chunks[2]);
        f.render_widget(Clear, area);
        f.render_widget(instructions, area);
    }

    if app.show_word_popup {
        let reveal_word = Paragraph::new(format!("Word:\n{}", app.wordle.word));
        let area = centered_rect(60, 40, outer_chunks[0]);
        f.render_widget(Clear, area);
        f.render_widget(reveal_word, area);
    }
}

fn string_to_styled_span(s: &str, app: &App, restart_keys: bool) -> Spans<'static> {
    Spans::from(
        s.chars()
            .map(|c| {
                let mut color = Color::White;
                match restart_keys {
                    false => {
                        if app.wordle.good_guess.contains(&c) {
                            color = Color::Yellow
                        } 
                        if app.wordle.bad_guess.contains(&c) {
                            color = Color::Black
                        } 
                        if app.wordle.perfect_guess.contains(&c) {
                            color = Color::Green
                        }
                    },
                    true => (),
                }
                
                Span::styled(c.to_string() + " ", Style::default().fg(color))
            })
            .collect::<Vec<Span>>(),
    )
}

fn gen_constraints(n: usize, c: Constraint) -> Vec<Constraint> {
    vec![c; n]
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

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
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum AppMode {
    Normal,
    Editing,
}

struct App {
    input: String,
    app_mode: AppMode,
    wordle: Wordle,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            app_mode: AppMode::Normal,
            wordle: Wordle::default()
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
            match app.app_mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.app_mode = AppMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                AppMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.wordle.submit_and_test_guess(app.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.app_mode = AppMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.app_mode {
        AppMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        AppMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };

    match app.app_mode {
        AppMode::Normal =>
            {}

        AppMode::Editing => {
            f.set_cursor(
                chunks[1].x + app.input.width() as u16 + 1,
                chunks[1].y + 7,
            )
        }
    }

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.app_mode {
            AppMode::Normal => Style::default(),
            AppMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, chunks[2]);

}
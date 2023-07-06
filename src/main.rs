use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Row, Table, TableState},
    Frame, Terminal,
};

#[derive(Debug, Clone)]
enum Player {
    One,
    Two,
}

#[derive(Debug, Clone)]
struct GameState {
    active_player: Player,
    selected_cell: TableState,
    can_place: bool,
    can_turn: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            active_player: Player::One,
            selected_cell: TableState::default(),
            can_place: true,
            can_turn: true,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut state = GameState::new();

    Ok(loop {
        terminal.draw(|frame| ui(frame, &mut state))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                let movement = match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                };
            }
        }
    })
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut GameState) {
    let frame_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    top_bar_ui(f, frame_chunks[0], &state);
    board_ui(f, frame_chunks[1], state);
}

fn top_bar_ui<B: Backend>(f: &mut Frame<B>, target: Rect, state: &GameState) {
    let top_bar_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(60),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ])
        .split(target);

    let active_player_label = Block::default().title("Active player: ");
    f.render_widget(active_player_label, top_bar_chunks[0]);

    let active_player = Block::default().title(format!("{:?}", state.active_player));
    f.render_widget(active_player, top_bar_chunks[1]);

    let place_label = Block::default().title(if state.can_place {
        "Can place"
    } else {
        "Can't place"
    });
    f.render_widget(place_label, top_bar_chunks[3]);

    let turn_label = Block::default().title(if state.can_turn {
        "Can turn"
    } else {
        "Can't turn"
    });
    f.render_widget(turn_label, top_bar_chunks[4]);
}

fn board_ui<B: Backend>(f: &mut Frame<B>, target: Rect, state: &mut GameState) {
    let board = Table::new(vec![
        Row::new([".", ".", ".", "|", ".", ".", "."]),
        Row::new([".", ".", ".", "|", ".", ".", "."]),
        Row::new([".", ".", ".", "|", ".", ".", "."]),
        Row::new(["-", "-", "-", "|", "-", "-", "-"]),
        Row::new([".", ".", ".", "|", ".", ".", "."]),
        Row::new([".", ".", ".", "|", ".", ".", "."]),
        Row::new([".", ".", ".", "|", ".", ".", "."]),
    ])
    // You can set the style of the entire Table.
    .style(Style::default().fg(Color::White))
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Board"))
    // Columns widths are constrained in the same way as Layout...
    .widths(&[
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // If you wish to highlight a row in any specific way when it is selected...
    .highlight_style(Style::default().fg(Color::Blue));

    f.render_stateful_widget(board, target, &mut state.selected_cell);
}

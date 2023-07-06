use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Row, Table, TableState},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state = GameState::new();
    state.selected_cell.select(Some(0));

    loop {
        terminal.draw(|f| ui(f, &mut state))?;

        if let Event::Key(key) = event::read()? {
            let movement = match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('w') => -7,
                KeyCode::Char('a') => -1,
                KeyCode::Char('s') => 1,
                KeyCode::Char('d') => 7,
                _ => 0,
            };

            state.selected_cell.select(Some(
                ((state.selected_cell.selected().unwrap() as i32 + movement) % 49) as usize,
            ));
        }
    }
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

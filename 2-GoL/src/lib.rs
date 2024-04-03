mod game_state;
mod infinite;

use game_state::GameState;

use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
    fs,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

pub fn run(file_path: &String) -> io::Result<()> {
    let contents = fs::read_to_string(file_path)
    .expect(&format!("Cannot read file {}", file_path));
    App::run(contents)
}

struct App<'a> {
    tick_count: u64,
    marker: Marker,
    commands: Paragraph<'a>,
    commands_dim: (u16, u16),
    game_state: GameState,
    offset: (isize, isize),
}

impl<'a> App<'a> {
    fn new(state: String) -> Self {
        Self {
            tick_count: 0,
            marker: Marker::Block,
            commands: Paragraph::new("Q - Quit\nP - Pause\nU - Unpause\nN - Next frame\nArrows - Move\n").block(
                Block::new()
                    .title("Commands")
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .style(Style::default()),
            ),
            commands_dim: (17, 7),
            game_state: GameState::new(state),
            offset: (0, 0),
        }
    }

    pub fn run(state: String) -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new(state);
        let mut last_tick = Instant::now();

        let tick_rate = Duration::from_millis(16);
        let mut mut_tick_rate = tick_rate;

        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('p') => mut_tick_rate = Duration::from_secs(86400),
                        KeyCode::Char('u') => mut_tick_rate = tick_rate,
                        KeyCode::Char('n') => (app.game_state, _) = app.game_state.next_state(),
                        KeyCode::Left => app.offset.0 -= 1,
                        KeyCode::Right => app.offset.0 += 1,
                        KeyCode::Down => app.offset.1 -= 1,
                        KeyCode::Up => app.offset.1 += 1,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= mut_tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        self.game_state = self.game_state.next_state().0;
    }

    fn ui(&self, frame: &mut Frame) {
        let frame_size = frame.size();

        let tl = (
            frame_size.x as isize + self.offset.0,
            frame_size.y as isize + self.offset.1,
        );
        let br = (
            (frame_size.x + frame_size.width) as isize + self.offset.0,
            (frame_size.y + frame_size.height) as isize + self.offset.1,
        );

        frame.render_widget(
            self.gol_canvas(frame_size, self.game_state.get_slice(tl, br)),
            frame_size,
        );

        let instructions_rect = Rect {
            x: frame_size.width - self.commands_dim.0,
            width: self.commands_dim.0,
            y: 0,
            height: self.commands_dim.1,
        };

        frame.render_widget(&self.commands, instructions_rect);
    }

    fn gol_canvas(&self, area: Rect, elements: Vec<(isize, isize)>) -> impl Widget + '_ {
        let elements: Vec<(f64, f64)> = elements
            .iter()
            .map(|&(x, y)| (x as f64, y as f64))
            .collect();

        Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Conway's Game of Life"),
            )
            .marker(self.marker)
            .paint(move |ctx| {
                ctx.draw(&Points {
                    coords: &elements,
                    color: Color::Green,
                });
            })
            .x_bounds([area.x as f64, area.width as f64])
            .y_bounds([area.y as f64, area.height as f64])
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

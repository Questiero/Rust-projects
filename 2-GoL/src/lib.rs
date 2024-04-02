mod game_state;
mod infinite;

use game_state::GameState;

use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
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

pub fn run() -> io::Result<()> {
    App::run()
}

struct App<'a> {
    tick_count: u64,
    marker: Marker,
    instructions: Paragraph<'a>,
    instructions_dim: (u16, u16),
    game_state: GameState,
    offset: (isize, isize),
}

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            tick_count: 0,
            marker: Marker::Block,
            instructions: Paragraph::new("Q - Quit\nP - Pause\nArrows - Move\n").block(
                Block::new()
                    .title("Instructions")
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .style(Style::default()),
            ),
            instructions_dim: (16, 5),
            game_state: GameState::new_empty(),
            offset: (0, 0),
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new();
        let mut last_tick = Instant::now();

        let tick_rate = Duration::from_millis(160);
        let mut mut_tick_rate = tick_rate;

        // TODO File input/Drawing
        app.game_state.grid.add_or_update(true, 10, 9);
        app.game_state.grid.add_or_update(true, 10, 10);
        app.game_state.grid.add_or_update(true, 10, 11);
        app.game_state.grid.add_or_update(true, 11, 9);
        app.game_state.grid.add_or_update(true, 9, 10);

        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('p') => {
                            if mut_tick_rate == tick_rate {
                                mut_tick_rate = Duration::from_secs(86400);
                            } else {
                                mut_tick_rate = tick_rate;
                            }
                        }
                        KeyCode::Down => app.offset.0 += 1,
                        KeyCode::Up => app.offset.0 -= 1,
                        KeyCode::Right => app.offset.0 += 1,
                        KeyCode::Left => app.offset.0 -= 1,
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
            x: frame_size.width - self.instructions_dim.0,
            width: self.instructions_dim.0,
            y: 0,
            height: self.instructions_dim.1,
        };

        frame.render_widget(&self.instructions, instructions_rect);
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

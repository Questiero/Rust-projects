mod infinite;

use core::fmt;
use infinite::Infinite2DMatrix;

#[derive(Debug, PartialEq, Eq)]
struct GameState {
    dim: (usize, usize),
    grid: Vec<Vec<bool>>,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_str = String::new();

        for line in &self.grid {
            for cell in line {
                if *cell {
                    fmt_str.push('■');
                } else {
                    fmt_str.push('□');
                }
            }
            fmt_str.push('\n');
        }

        write!(f, "{fmt_str}")
    }
}

impl GameState {
    fn next_state(&self) -> (GameState, bool) {
        let mut next_state = GameState::new_empty(self.dim);

        for i in 0..(self.dim.0) {
            for j in 0..(self.dim.1) {
                if self.is_cell_alive(i, j) {
                    next_state.grid[i][j] = true;
                }
            }
        }

        let is_static = *self == next_state;

        (next_state, is_static)
    }

    fn is_cell_alive(&self, i: usize, j: usize) -> bool {
        let mut neighbors: Vec<bool> = Vec::new();

        {
            let i = i as isize;
            let j = j as isize;

            for k in [i - 1, i, i + 1] {
                for l in [j - 1, j, j + 1] {
                    if ((k, l) != (i, j))
                        & (k >= 0)
                        & ((k as usize) < self.dim.0)
                        & (l >= 0)
                        & ((l as usize) < self.dim.1)
                    {
                        neighbors.push(self.grid[k as usize][l as usize]);
                    }
                }
            }
        }

        let alive_neighbors = neighbors.iter().filter(|&c| *c).count();

        ((self.grid[i][j]) & (alive_neighbors == 2)) | (alive_neighbors == 3)
    }

    fn new_empty(dim: (usize, usize)) -> GameState {
        GameState {
            dim,
            grid: vec![vec![false; dim.0]; dim.1],
        }
    }
}

pub fn run() {
    let mut state = GameState {
        dim: (10, 10),
        grid: vec![
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, true, true, false,
            ],
            vec![
                false, false, false, false, false, false, false, true, false, true,
            ],
            vec![
                false, false, false, false, false, false, false, true, false, false,
            ],
        ],
    };

    let mut is_static = false;

    while !is_static {
        print!("\x1B[2J\x1b[1;1H");
        print!("{state}");

        (state, is_static) = state.next_state();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

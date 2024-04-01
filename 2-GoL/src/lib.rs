use core::fmt;

#[derive(PartialEq, Eq)]
struct GameState {
    grid: [[i8; 10]; 10],
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_str = String::new();

        for line in self.grid {
            for cell in line {
                if cell == 1 {
                    fmt_str.push_str("■");
                } else if cell == 0 {
                    fmt_str.push_str("□");
                } else {
                    panic!("Cell's value should be either 0 or 1.");
                }
            }
            fmt_str.push_str("\n");
        }

        write!(f, "{fmt_str}")
    }
}

impl GameState {
    fn next_state(&self) -> (GameState, bool) {
        let dimensions = (self.grid.len(), self.grid[0].len());

        // Make sure to use dimensions.0 and .1 instead of hardcoding when switching to other structures
        let mut next_state = GameState {
            grid: [[0; 10]; 10],
        };

        for i in 0..(dimensions.0) {
            for j in 0..(dimensions.1) {
                if self.is_cell_alive(i, j) {
                    next_state.grid[i][j] = 1;
                }
            }
        }

        let is_static = *self == next_state;

        (next_state, is_static)
    }

    fn is_cell_alive(&self, i: usize, j: usize) -> bool {
        let dimensions = (self.grid.len(), self.grid[0].len());

        let mut neighbors: Vec<i8> = Vec::new();

        {
            let i = i as isize;
            let j = j as isize;

            for k in [i - 1, i, i + 1] {
                for l in [j - 1, j, j + 1] {
                    if ((k, l) != (i, j))
                        & (k >= 0)
                        & (k as usize <= dimensions.0 - 1)
                        & (l >= 0)
                        & (l as usize <= dimensions.1 - 1)
                    {
                        neighbors.push(self.grid[k as usize][l as usize]);
                    }
                }
            }
        }

        return ((self.grid[i][j] == 1) & (neighbors.iter().sum::<i8>() == 2))
            | (neighbors.iter().sum::<i8>() == 3);
    }
}

pub fn run() {
    let mut state = GameState {
        grid: [
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
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

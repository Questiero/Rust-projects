mod infinite;

use core::fmt;
use std::isize;
use infinite::Infinite2DMatrix;
use terminal_size;

#[derive(Debug, PartialEq, Eq)]
struct GameState {
    offset: (isize, isize),
    grid: Infinite2DMatrix<bool>,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (screen_width, screen_height) = get_terminal_size();
        let br = (screen_width + self.offset.0 - 2, screen_height + self.offset.1 - 2);

        let mut disp = "-".repeat(screen_width as usize);

        for line in self.get_slice(self.offset, br).lines() {
            disp.push_str(&format!("|{line}|"));
        }
        disp.push_str(&"-".repeat(screen_width as usize));

        write!(f, "{disp}")
    }
}

impl GameState {
    #[allow(dead_code)]
    fn next_state(&self) -> (GameState, bool) {
        let mut next_state = GameState::new_empty();

        let mut nb_neighbors: Infinite2DMatrix<i8> = Infinite2DMatrix::new();

        for ((i, j), _cell) in self.grid.elements() {
            for k in [i - 1, i, i + 1] {
                for l in [j - 1, j, j + 1] {
                    if (k, l) != (i, j) {
                        match nb_neighbors.get(&k, &l) {
                            None => {
                                nb_neighbors.add_or_update(1, k, l);
                            }
                            Some(n) => {
                                nb_neighbors.add_or_update(n + 1, k, l);
                            }
                        }
                    }
                }
            }
        }

        for ((i, j), nb_ngh) in nb_neighbors.elements() {
            if (nb_ngh == 3) || (nb_ngh == 2 && self.grid.get(&i, &j).is_some()) {
                next_state.grid.add_or_update(true, i, j);
            }
        }

        let is_static = *self == next_state;
        (next_state, is_static)
    }

    #[allow(dead_code)]
    fn new_empty() -> GameState {
        GameState {
            offset: (0, 0),
            grid: Infinite2DMatrix::new(),
        }
    }

    #[allow(dead_code)]
    fn get_slice(&self, tl: (isize, isize), br: (isize, isize)) -> String {
        let mut slice = String::new();

        for i in tl.1..br.1 {
            for j in tl.0..br.0 {
                match self.grid.get(&i, &j) {
                    None => {slice.push(' ');},
                    Some(_) => {slice.push('0');},
                }
            }
            slice.push('\n');
        }

        slice
    }
}

pub fn run() {
    
    let mut game_state = GameState::new_empty();
    game_state.grid.add_or_update(true, 10, 9);
    game_state.grid.add_or_update(true, 10, 10);
    game_state.grid.add_or_update(true, 10, 11);
    game_state.grid.add_or_update(true, 11, 9);
    game_state.grid.add_or_update(true, 9, 10);



    print!("\x1B[2J\x1B[1;1H{}", game_state);

    loop {
        game_state = game_state.next_state().0;
        print!("\x1B[2J\x1B[1;1H{}", game_state);
    
        std::thread::sleep(std::time::Duration::from_millis(250));
    }

}

fn get_terminal_size() -> (isize, isize) {
    let size = terminal_size::terminal_size();
    let mut width = 0;
    let mut height = 0;
    if let Some((terminal_size::Width(w), terminal_size::Height(h))) = size {
        width = w;
        height = h;
    } else {
        println!("Unable to get terminal size");
    }
    (width as isize, height as isize)
}
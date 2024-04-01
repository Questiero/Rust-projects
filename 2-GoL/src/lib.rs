mod infinite;

use core::fmt;
use infinite::Infinite2DMatrix;

#[derive(Debug, PartialEq, Eq)]
struct GameState {
    grid: Infinite2DMatrix<bool>,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
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
                        match nb_neighbors.get(k, l) {
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
            if (nb_ngh == 3) || (nb_ngh == 2 && self.grid.get(i, j).is_some()) {
                next_state.grid.add_or_update(true, i, j);
            }
        }

        let is_static = *self == next_state;
        (next_state, is_static)
    }

    #[allow(dead_code)]
    fn new_empty() -> GameState {
        GameState {
            grid: Infinite2DMatrix::new(),
        }
    }
}

pub fn run() {
    
    let mut game_state = GameState::new_empty();
    game_state.grid.add_or_update(true, 0, 0);
    game_state.grid.add_or_update(true, 0, 1);
    game_state.grid.add_or_update(true, 0, -1);

    println!("{game_state}");

    loop {
        game_state = game_state.next_state().0;
        println!("{game_state}");
    
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}

use crate::infinite::Infinite2DMatrix;
use std::{fmt::Debug, isize};

#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    pub grid: Infinite2DMatrix<bool>,
}

impl GameState {
    #[allow(dead_code)]
    pub fn next_state(&self) -> (GameState, bool) {
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
    pub fn new_empty() -> GameState {
        GameState {
            grid: Infinite2DMatrix::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_slice(&self, tl: (isize, isize), br: (isize, isize)) -> Vec<(isize, isize)> {
        let mut slice = Vec::new();

        for i in tl.0..br.0 {
            for j in tl.1..br.1 {
                match self.grid.get(&i, &j) {
                    None => {}
                    Some(_) => {
                        slice.push((i-tl.0, j-tl.1));
                    }
                }
            }
        }

        slice
    }
}

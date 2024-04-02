use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Infinite2DMatrix<T: Copy> {
    len: usize,
    map: HashMap<isize, HashMap<isize, T>>,
}

impl<T: Copy> Infinite2DMatrix<T> {
    #[allow(dead_code)]
    pub fn new() -> Infinite2DMatrix<T> {
        Infinite2DMatrix {
            len: 0,
            map: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, i: &isize, j: &isize) -> Option<T> {
        let value: T;

        match self.map.get(i) {
            None => return None,
            Some(r) => match r.get(j) {
                None => return None,
                Some(c) => value = *c,
            },
        };

        Some(value)
    }

    #[allow(dead_code)]
    pub fn add_or_update(&mut self, elem: T, i: isize, j: isize) {
        match self.map.get_mut(&i) {
            None => {
                let mut col: HashMap<isize, T> = HashMap::new();
                col.insert(j, elem);
                self.map.insert(i, col);
            }
            Some(col) => match col.get(&j) {
                None => {
                    col.insert(j, elem);
                    self.len += 1;
                }
                Some(_) => {
                    col.remove(&j);
                    col.insert(j, elem);
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, i: &isize, j: &isize) {
        match self.map.get_mut(i) {
            None => {}
            Some(col) => match col.get(j) {
                None => {}
                Some(_) => {
                    col.remove(j);
                    if col.is_empty() {
                        self.map.remove(i);
                        self.len -= 1;
                    }
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[allow(dead_code)]
    pub fn elements(&self) -> Vec<((isize, isize), T)> {
        let mut elements = vec![];

        for i in self.map.keys() {
            for j in self.map[i].keys() {
                elements.push(((*i, *j), self.map[i][j]));
            }
        }

        elements
    }
}

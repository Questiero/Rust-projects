// ===== Rod =====

#[derive(Debug)]
struct Rod {
    discs: Vec<u8>,
}

impl Rod {
    fn new() -> Self {
        Self { discs: Vec::new() }
    }
    fn from_vec(discs: Vec<u8>) -> Self {
        Self { discs }
    }
    fn size(&self) -> u8 {
        self.discs.len() as u8
    }
}

// ===== Tower =====

#[derive(Debug)]
struct Tower {
    rods: Vec<Rod>,
}

impl Tower {
    fn size(&self) -> u8 {
        self.rods.iter().map(|r| -> u8 { r.size() }).sum()
    }

    fn move_disc(&mut self, src: usize, tgt: usize) {
        let val = self.rods[src]
            .discs
            .pop()
            .expect("We only move from a rod with discs");
        self.rods[tgt].discs.push(val);
    }
}

impl std::fmt::Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print = String::new();
        let size = self.size() as usize;

        let rod_slice = |rod: &Rod, index: usize, size: usize| -> String {
            let half_disc = match rod.discs.get(index) {
                Some(n) => "=".repeat(*n as usize),
                None => "".to_string(),
            };
            format!("{half_disc:>size$}|{half_disc:<size$}")
        };

        for i in (0..(size)).rev() {
            let mut line_str: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
            for (j, s) in line_str.iter_mut().enumerate() {
                *s = rod_slice(&self.rods[j], i, size).to_string();
            }
            print.push_str(&format!("{}\n", line_str.join(" ")));
        }

        write!(f, "{print}")
    }
}

// ===== functions =====

pub fn run(n: u8) {
    let tower = Tower {
        rods: vec![
            Rod::from_vec((1..n + 1).rev().collect()),
            Rod::new(),
            Rod::new(),
        ],
    };

    solve(tower);
}

fn solve(mut tower: Tower) {
    println!("0 - Initial state of the tower");
    println!("{}", tower);

    let tower_size = tower.size();
    let is_pair = tower_size % 2 == 0;

    // Invert rod 1 and 2 if number of discs is pair
    let swap_end_if_pair = |n: usize| -> usize {
        if is_pair {
            if n == 1 {
                return 2;
            };
            if n == 2 {
                return 1;
            };
        }
        n
    };

    let mut m = 0b1;

    while tower_size != tower.rods[2].size() {
        let src = swap_end_if_pair((m & (m - 1)) % 3);
        let tgt = swap_end_if_pair(((m | (m - 1)) + 1) % 3);

        println!("{m} - Move disc from rod {src} to rod {tgt}");
        tower.move_disc(src, tgt);
        println!("{}", tower);

        m += 1;
    }
}

// ===== tests =====

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tower_size() {
        let tower = Tower {
            rods: vec![
                Rod::from_vec(vec![5, 3, 2]),
                Rod::from_vec(vec![4, 1]),
                Rod::new(),
            ],
        };

        assert_eq!(tower.size(), 5);
    }
}

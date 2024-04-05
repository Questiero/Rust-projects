#[derive(Debug)]
struct Stack {
    cells: [u8; 30000],
    pointer: usize,
}

impl Stack {
    fn new() -> Self {
        Stack {
            cells: [0; 30000],
            pointer: 0,
        }
    }

    fn increment_pointer(&mut self) {
        self.pointer += 1;
    }

    fn decrement_pointer(&mut self) {
        self.pointer -= 1;
    }

    fn increment_cell(&mut self) {
        self.cells[self.pointer] += 1;
    }

    fn decrement_cell(&mut self) {
        self.cells[self.pointer] -= 1;
    }

    fn get_cell_byte(&self) -> u8 {
        self.cells[self.pointer]
    }

    fn current_cell_value(&self) -> u8 {
        self.cells[self.pointer]
    }
}

pub fn interpret(str_instructions: String) {
    println!("{}", str_instructions);

    let mut stack = Stack::new();

    let instructions: Vec<char> = str_instructions.chars().collect();
    let instructions_len = instructions.len();
    let mut pointer = 0;

    while pointer < instructions_len {
        match instructions.get(pointer).expect("Pointer out of bounds.") {
            '>' => stack.increment_pointer(),
            '<' => stack.decrement_pointer(),
            '+' => stack.increment_cell(),
            '-' => stack.decrement_cell(),
            '.' => print!("{}", stack.get_cell_byte() as char),
            ',' => panic!("',' not implemented."),
            '[' => {
                if stack.current_cell_value() == 0 {
                    let partial_bracket_index = str_instructions[pointer..]
                        .find(']')
                        .expect("Matching ']' not found.");
                    pointer += partial_bracket_index;
                    continue;
                }
            }
            ']' => {
                if stack.current_cell_value() != 0 {
                    let partial_string = str_instructions[..pointer]
                        .chars()
                        .rev()
                        .collect::<String>();
                    let partial_bracket_index =
                        partial_string.find('[').expect("Matching '[' not found.");
                    pointer = partial_string.len() - partial_bracket_index;
                    continue;
                }
            }
            _ => {}
        }

        pointer += 1;
    }
}

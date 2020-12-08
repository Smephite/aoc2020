#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    ACC,
    JMP,
    NOP,
}

impl Operation {
    pub fn find(st: &str) -> Self {
        match st {
            "acc" => Self::ACC,
            "jmp" => Self::JMP,
            _ => Self::NOP,
        }
    }
}

#[derive(Debug)]
pub struct Processor {
    pointer: usize,
    accumulator: isize,
    executed_pointers: Vec<usize>,
    instructions: Vec<(Operation, Vec<isize>)>,
    exit_code: isize,
}

impl Default for Processor {
    fn default() -> Self {
        Self {
            pointer: 0,
            accumulator: 0,
            executed_pointers: vec![],
            instructions: vec![],
            exit_code: 1,
        }
    }
}

impl Processor {
    fn execute(&mut self, op: &Operation, arg: &Vec<isize>, duplicated: bool) -> bool {
        if !duplicated && self.executed_pointers.contains(&self.pointer) {
            self.exit_code = -1;
            return false;
        }

        self.executed_pointers.push(self.pointer);

        match op {
            Operation::ACC => {
                self.accumulator += arg[0];
                self.pointer += 1;
            }
            Operation::JMP => {
                let jmp = arg[0];
                if jmp < 0 {
                    assert_eq!(self.pointer < (-jmp) as usize, false);
                    self.pointer -= (-jmp) as usize;
                } else {
                    self.pointer += jmp as usize;
                }
            }
            _ => self.pointer += 1, // Noop
        };

        return true;
    }

    pub fn load(&mut self, code: &Vec<(Operation, Vec<isize>)>) -> &Processor {
        self.instructions = code.to_vec();

        self
    }

    pub fn run_processor(&mut self, dup: bool) -> &Processor {
        loop {
            if self.pointer >= self.instructions.len() {
                self.exit_code = 0;
                break;
            }

            let instr = self.instructions[self.pointer].clone();

            if !self.execute(&instr.0, &instr.1, dup) {
                break;
            }
        }

        self
    }

    pub fn run_no_duplicate(&mut self) -> &Processor {
        self.run_processor(false)
    }
    #[allow(dead_code)]
    pub fn run(&mut self) -> &Processor {
        self.run_processor(true)
    }

    pub fn get_accumulator(&self) -> isize {
        self.accumulator
    }

    #[allow(dead_code)]
    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn get_exit_code(&self) -> isize {
        self.exit_code
    }
}

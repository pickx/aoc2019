use std::collections::VecDeque;

#[derive(Eq, PartialEq)]
pub enum Opcode {
    Add(Value, Value, usize),      //1
    Mul(Value, Value, usize),      //2
    Input(usize),                  //3
    Output(usize),                 //4
    JumpIfTrue(Value, Value),      //5
    JumpIfFalse(Value, Value),     //6
    LessThan(Value, Value, usize), //7
    Equals(Value, Value, usize),   //8
    Halt,                          //99
}

impl Opcode {
    fn num_vals(&self) -> usize {
        match self {
            Opcode::Add(_, _, _)
            | Opcode::Mul(_, _, _)
            | Opcode::LessThan(_, _, _)
            | Opcode::Equals(_, _, _) => 3,
            Opcode::Input(_) | Opcode::Output(_) => 1,
            Opcode::JumpIfTrue(_, _) | Opcode::JumpIfFalse(_, _) => 2,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Value {
    Position(usize),  //0
    Immediate(isize), //1
}

impl Value {
    fn eval(self, mem: &[isize]) -> isize {
        match self {
            Value::Position(pos) => mem[pos],
            Value::Immediate(imm) => imm,
        }
    }

    fn create(mem_contents: isize, mode_code: isize) -> Value {
        match mode_code {
            0 => Value::Position(mem_contents as usize),
            1 => Value::Immediate(mem_contents),
            _ => panic!("Unsupported parameter mode."),
        }
    }
}

pub struct CodeRunner {
    mem: Vec<isize>,
    inst_ptr: usize,
    inputs: VecDeque<isize>,
    input_mode: InputMode,
    output: Option<isize>,
    halted: bool,

}

pub enum InputMode {
    ConsumeInput,
    SingleInput,
}

impl CodeRunner {

    pub fn new(mem: &[isize]) -> CodeRunner {
        CodeRunner { mem: mem.to_vec(), inst_ptr: 0, inputs: VecDeque::new(), input_mode: InputMode::ConsumeInput, output: None, halted: false, }
    }

    fn get_next_input(&mut self) -> isize {

        match self.input_mode {
            InputMode::ConsumeInput => self.inputs.pop_back().expect("Input is empty"),
            InputMode::SingleInput => self.inputs.back().expect("Input is empty").clone(),
        }
    }

//    pub fn clear_output(&mut self) {
//        self.output = None;
//    }

    pub fn push_input_front(&mut self, input: isize) {
        self.inputs.push_front(input);
    }

    pub fn push_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    pub fn set_input_consume_mode(&mut self, mode: InputMode) { self.input_mode = mode; }

    pub fn output(&self) -> Option<isize> { self.output }

    pub fn value_at_pos_0(&self) -> isize { self.mem[0] }

    pub fn set_noun(&mut self, noun: isize) { self.mem[1] = noun; }

    pub fn set_verb(&mut self, verb: isize) { self.mem[2] = verb; }

    pub fn has_halted(&self) -> bool { self.halted }

//    pub fn restart(&mut self, mem: &[isize]) {
//        self.mem = mem.to_vec();
//
//        self.inst_ptr = 0;
//        self.halted = false;
//    }

    pub fn parse_cur_opcode(&self) -> Opcode {
        let code = self.mem[self.inst_ptr];

        match code % 100 {
            1 => {
                let val1 = Value::create(self.mem[self.inst_ptr + 1], (code / 100) % 10);
                let val2 = Value::create(self.mem[self.inst_ptr + 2], (code / 1000) % 10);
                let dest = self.mem[self.inst_ptr + 3] as usize;

                Opcode::Add(val1, val2, dest)
            }

            2 => {
                let val1 = Value::create(self.mem[self.inst_ptr + 1], (code / 100) % 10);
                let val2 = Value::create(self.mem[self.inst_ptr + 2], (code / 1000) % 10);
                let dest = self.mem[self.inst_ptr + 3] as usize;

                Opcode::Mul(val1, val2, dest)
            }

            3 => {
                let dest = self.mem[self.inst_ptr + 1] as usize;

                Opcode::Input(dest)
            }

            4 => {
                let dest = self.mem[self.inst_ptr + 1] as usize;

                Opcode::Output(dest)
            }

            5 => {
                let val1 = Value::create(self.mem[self.inst_ptr + 1], (code / 100) % 10);
                let val2 = Value::create(self.mem[self.inst_ptr + 2], (code / 1000) % 10);

                Opcode::JumpIfTrue(val1, val2)
            }

            6 => {
                let val1 = Value::create(self.mem[self.inst_ptr + 1], (code / 100) % 10);
                let val2 = Value::create(self.mem[self.inst_ptr + 2], (code / 1000) % 10);

                Opcode::JumpIfFalse(val1, val2)
            }

            7 => {
                let val1 = Value::create(self.mem[self.inst_ptr + 1], (code / 100) % 10);
                let val2 = Value::create(self.mem[self.inst_ptr + 2], (code / 1000) % 10);
                let dest = self.mem[self.inst_ptr + 3] as usize;

                Opcode::LessThan(val1, val2, dest)
            }

            8 => {
                let val1 = Value::create(self.mem[self.inst_ptr + 1], (code / 100) % 10);
                let val2 = Value::create(self.mem[self.inst_ptr + 2], (code / 1000) % 10);
                let dest = self.mem[self.inst_ptr + 3] as usize;

                Opcode::Equals(val1, val2, dest)
            }

            99 => Opcode::Halt,

            _ => panic!("Unsupported opcode: {}", code),
        }
    }

    pub fn run_opcode(&mut self, opcode: Opcode) {
        self.inst_ptr += &opcode.num_vals() + 1;

        match opcode {
            Opcode::Add(val1, val2, dest) => {
                self.mem[dest] = val1.eval(&self.mem) + val2.eval(&self.mem)
            }

            Opcode::Mul(val1, val2, dest) => {
                self.mem[dest] = val1.eval(&self.mem) * val2.eval(&self.mem)
            }

            Opcode::Input(dest) => self.mem[dest] = self.get_next_input(),

            Opcode::Output(dest) => self.output = Some(self.mem[dest]),

            Opcode::JumpIfTrue(val1, val2) => {
                if val1.eval(&self.mem) != 0 {
                    self.inst_ptr = val2.eval(&self.mem) as usize;
                }
            }

            Opcode::JumpIfFalse(val1, val2) => {
                if val1.eval(&self.mem) == 0 {
                    self.inst_ptr = val2.eval(&self.mem) as usize;
                }
            }

            Opcode::LessThan(val1, val2, dest) => {
                let comparison_res = (val1.eval(&self.mem) < val2.eval(&self.mem)) as isize;
                self.mem[dest] = comparison_res;
            }

            Opcode::Equals(val1, val2, dest) => {
                let comparison_res = (val1.eval(&self.mem) == val2.eval(&self.mem)) as isize;
                self.mem[dest] = comparison_res;
            }

            Opcode::Halt => self.halted = true,
        };
    }
}

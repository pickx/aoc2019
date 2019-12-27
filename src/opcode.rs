use std::collections::{HashMap, VecDeque};
use std::io;
use std::fmt::Error;
use std::ops::Deref;

#[derive(Eq, PartialEq, Clone)]
pub enum Opcode {
    Add(Value, Value, Value),  //1
    Mul(Value, Value, Value),  //2
    In(Value),                 //3
    Out(Value),                //4
    JumpIfTrue(Value, Value),  //5
    JumpIfFalse(Value, Value), //6
    LT(Value, Value, Value),   //7
    EQ(Value, Value, Value),   //8
    BaseOffset(Value),         //9
    Halt,                      //99
    Reboot,                    //0
}

impl Opcode {
    fn num_vals(&self) -> isize {
        match self {
            Opcode::Add(_, _, _)
            | Opcode::Mul(_, _, _)
            | Opcode::LT(_, _, _)
            | Opcode::EQ(_, _, _) => 3,
            Opcode::In(_) | Opcode::Out(_) | Opcode::BaseOffset(_) => 1,
            Opcode::JumpIfTrue(_, _) | Opcode::JumpIfFalse(_, _) => 2,
            Opcode::Halt => 0,
            Opcode::Reboot => 0,
        }
    }
}



#[derive(Eq, PartialEq, Clone)]
pub enum Value {
    Position(isize),  //0
    Immediate(isize), //1
    Relative(isize),  //2
}

impl Value {
    pub fn new(mem_contents: isize, mode_code: isize) -> Value {
        match mode_code {
            0 => Value::Position(mem_contents),
            1 => Value::Immediate(mem_contents),
            2 => Value::Relative(mem_contents),
            _ => panic!("Unsupported parameter mode."),
        }
    }
}

#[derive(Clone)]
pub struct OpcodeRunner {
    mem: Vec<isize>,
    extra_mem: HashMap<isize, isize>,
    inst_ptr: isize,
    offset: isize,
    inputs: VecDeque<isize>,
    input_mode: InputMode,
    output: Option<isize>,
    halted: bool,
}

#[derive(Clone)]
pub enum InputMode {
    ConsumeInput,
    SingleInput,
}

impl OpcodeRunner {
    pub fn new(mem: &[isize]) -> OpcodeRunner {
        OpcodeRunner {
            mem: mem.to_vec(),
            extra_mem: HashMap::new(),
            inst_ptr: 0,
            offset: 0,
            inputs: VecDeque::new(),
            input_mode: InputMode::ConsumeInput,
            output: None,
            halted: false,
        }
    }

/// this deconstructs "state" after use and drop()s all of self's old members.
/// it's basically a move constructor.
    pub fn load_state(&mut self, state: OpcodeRunner) {
        self.mem = state.mem;
        self.extra_mem = state.extra_mem;
        self.inst_ptr = state.inst_ptr;
        self.offset = state.offset;
        self.inputs = state.inputs;
        self.output = state.output;
        self.halted = state.halted;

    }

    #[allow(dead_code)]
    pub fn print_mem(&mut self) {
        dbg!(&self.mem);
        dbg!(&self.extra_mem);
        println!(
            "input is {:?} and offset is {}",
            self.inputs.back(),
            self.offset
        );
    }

    // unfortunately for now this has to take &mut self, because
    // accessing uninitialized positive memory is legal and would
    // require modifying the extra memory.
    fn mem_at(&mut self, addr: isize) -> isize {
        let mem_limit = self.mem.len() as isize;

        match addr {
            a if a < 0 => panic!("Attempt to access a negative memory address"),
            a if a < mem_limit => self.mem[addr as usize],
            _ => {
                self.extra_mem.entry(addr).or_default(); //default is 0
                self.extra_mem[&addr]
            }
        }
    }

    fn set_mem(&mut self, addr: isize, new_val: isize) {
        let mem_limit = self.mem.len() as isize;

        match addr {
            a if a < 0 => panic!("Attempt to access a negative memory address"),
            a if a < mem_limit => self.mem[addr as usize] = new_val,
            _ => {
                self.extra_mem.insert(addr, new_val);
            }
        }
    }

    fn get_next_input(&mut self) -> isize {
        match self.input_mode {
            InputMode::ConsumeInput => self.inputs.pop_back().expect("Input is empty"),
            InputMode::SingleInput => *self.inputs.back().expect("Input is empty"),
        }
    }

    //see: https://www.reddit.com/r/adventofcode/comments/e8aw9j/2019_day_9_part_1_how_to_fix_203_error/
    pub fn eval_interpret(&mut self, val: Value) -> isize {
        match val {
            Value::Position(addr) => self.mem_at(addr),
            Value::Immediate(imm) => imm,
            Value::Relative(rel_addr) => self.mem_at(self.offset + rel_addr),
        }
    }

    pub fn eval_literal(&mut self, val: Value) -> isize {
        match val {
            Value::Position(addr) => addr,
            Value::Immediate(imm) => imm, //should probably panic!() here
            Value::Relative(rel_addr) => self.offset + rel_addr,
        }
    }


    pub fn push_input_front(&mut self, input: isize) {
        self.inputs.push_front(input);
    }

    pub fn push_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    pub fn set_input_consume_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    pub fn output(&self) -> Option<isize> {
        self.output
    }

    pub fn value_at_pos_0(&mut self) -> isize {
        self.mem_at(0)
    }

    pub fn set_noun(&mut self, noun: isize) {
        self.set_mem(1, noun);
    }

    pub fn set_verb(&mut self, verb: isize) {
        self.set_mem(2, verb);
    }

    pub fn has_halted(&self) -> bool {
        self.halted
    }

    fn reboot(&mut self) {
        self.inst_ptr = 0;
        self.offset = 0;
        self.inputs.clear();
        self.extra_mem.clear();
        self.halted = false;
    }

    pub fn parse_cur_opcode(&mut self) -> Opcode {
        let ptr = self.inst_ptr;
        let code = self.mem_at(ptr);
        let two_digit_opcode = code % 100;
        let (mode1, mode2, mode3) = ((code / 100) % 10, (code / 1000) % 10, (code / 10000) % 10);

        let val1 = Value::new(self.mem_at(ptr + 1), mode1);
        let val2 = Value::new(self.mem_at(ptr + 2), mode2);
        let val3 = Value::new(self.mem_at(ptr + 3), mode3);

        match two_digit_opcode {
            1 => Opcode::Add(val1, val2, val3),

            2 => Opcode::Mul(val1, val2, val3),

            3 => Opcode::In(val1),

            4 => Opcode::Out(val1),

            5 => Opcode::JumpIfTrue(val1, val2),

            6 => Opcode::JumpIfFalse(val1, val2),

            7 => Opcode::LT(val1, val2, val3),

            8 => Opcode::EQ(val1, val2, val3),

            9 => Opcode::BaseOffset(val1),

            99 => Opcode::Halt,

            0 => Opcode::Reboot,

            _ => panic!("Unsupported opcode: {}", code),
        }
    }

    //executes opcode and returns true iff an Out instruction was executed
    pub fn exec_opcode(&mut self, opcode: Opcode) -> bool {
        self.inst_ptr += &opcode.num_vals() + 1;
        let mut got_new_output = false;

        match opcode {
            Opcode::Add(val1, val2, val3) => {
                let op1 = self.eval_interpret(val1);
                let op2 = self.eval_interpret(val2);
                let addr = self.eval_literal(val3);
                self.set_mem(addr, op1 + op2);
            }

            Opcode::Mul(val1, val2, val3) => {
                let op1 = self.eval_interpret(val1);
                let op2 = self.eval_interpret(val2);
                let addr = self.eval_literal(val3);
                self.set_mem(addr, op1 * op2);
            }

            Opcode::In(val) => {
                let input = self.get_next_input();
                let addr = self.eval_literal(val);
                self.set_mem(addr, input);

            }

            Opcode::Out(val) => {

                self.output = Some(self.eval_interpret(val));
                got_new_output = true;
            }

            Opcode::JumpIfTrue(val1, val2) => {
                if self.eval_interpret(val1) != 0 {
                    self.inst_ptr = self.eval_interpret(val2);
                }
            }

            Opcode::JumpIfFalse(val1, val2) => {
                if self.eval_interpret(val1) == 0 {
                    self.inst_ptr = self.eval_interpret(val2);
                }
            }

            Opcode::LT(val1, val2, val3) => {
                let comparison_res = (self.eval_interpret(val1) < self.eval_interpret(val2)) as isize;
                let addr = self.eval_literal(val3);
                self.set_mem(addr, comparison_res);
            }

            Opcode::EQ(val1, val2, val3) => {
                let comparison_res = (self.eval_interpret(val1) == self.eval_interpret(val2)) as isize;
                let addr = self.eval_literal(val3);
                self.set_mem(addr, comparison_res);
            }

            Opcode::BaseOffset(val) => {
                let offset_change = self.eval_interpret(val);
                self.offset += offset_change;
            }

            Opcode::Halt => self.halted = true,

            Opcode::Reboot => self.reboot(),
        };

        got_new_output
    }

    pub fn ask_for_input(lookup_table: &HashMap<char, isize>) -> Result<isize, &'static str> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let first_char = input
                    .chars()
                    .nth(0)
                    .unwrap();

                //wow this is ugly, gotta practice making these look better.
                lookup_table
                    .get(&first_char)
                    .and_then(|res| Some(*res))
                    .ok_or_else(|| "Command not found in lookup table")
            }
            Err(_error) => panic!("failed to read input, got {:?}", _error),
        }
    }
}
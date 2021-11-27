use std::{cell::RefCell, collections::HashMap};

const DEBUG: bool = false;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParamMode {
    Imm,
    Pos,
    Rel,
}
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add(ParamMode, ParamMode, ParamMode),
    Mul(ParamMode, ParamMode, ParamMode),
    Input(ParamMode),
    Output(ParamMode),
    JumpIfTrue(ParamMode, ParamMode),
    JumpIfFalse(ParamMode, ParamMode),
    LessThan(ParamMode, ParamMode, ParamMode),
    Equals(ParamMode, ParamMode, ParamMode),
    AdjustRelBase(ParamMode),
    Term,
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionEffect {
    NoEffect,
    OutputValue(i64),
    Jump(usize),
    WriteMem(usize, i64),
    MoveRelBase(i64),
    Halt,
}

use Instruction::*;
use InstructionEffect::*;
use ParamMode::*;

pub fn parse_param_mode(instruction: i64, param_n: u32) -> ParamMode {
    match ((instruction) / (10 * (10_i64).pow(param_n))) % 10 {
        0 => Pos,
        1 => Imm,
        2 => Rel,
        _ => panic!("invalid param mode"),
    }
}

pub fn parse_instruction(instruction: i64) -> Instruction {
    match instruction % 100 {
        1 => Add(
            parse_param_mode(instruction, 1),
            parse_param_mode(instruction, 2),
            parse_param_mode(instruction, 3),
        ),
        2 => Mul(
            parse_param_mode(instruction, 1),
            parse_param_mode(instruction, 2),
            parse_param_mode(instruction, 3),
        ),
        3 => Input(parse_param_mode(instruction, 1)),
        4 => Output(parse_param_mode(instruction, 1)),
        5 => JumpIfTrue(
            parse_param_mode(instruction, 1),
            parse_param_mode(instruction, 2),
        ),
        6 => JumpIfFalse(
            parse_param_mode(instruction, 1),
            parse_param_mode(instruction, 2),
        ),
        7 => LessThan(
            parse_param_mode(instruction, 1),
            parse_param_mode(instruction, 2),
            parse_param_mode(instruction, 3),
        ),
        8 => Equals(
            parse_param_mode(instruction, 1),
            parse_param_mode(instruction, 2),
            parse_param_mode(instruction, 3),
        ),
        9 => AdjustRelBase(parse_param_mode(instruction, 1)),
        99 => Term,
        _ => panic!("invalid instruction {}", instruction % 100),
    }
}

pub fn instruction_size(instruction: Instruction) -> usize {
    match instruction {
        Add(_, _, _) => 4,
        Mul(_, _, _) => 4,
        Input(_) => 2,
        Output(_) => 2,
        JumpIfTrue(_, _) => 3,
        JumpIfFalse(_, _) => 3,
        LessThan(_, _, _) => 4,
        Equals(_, _, _) => 4,
        AdjustRelBase(_) => 2,
        Term => 1,
    }
}

pub struct IntcodeComputer {
    program: RefCell<Vec<i64>>,
    inputs: RefCell<Vec<i64>>,
    vspace: RefCell<HashMap<usize, i64>>,
    outputs: RefCell<Vec<i64>>,
    pos: usize,
    relative_base: i64,
    halted: bool,
}

impl IntcodeComputer {
    pub fn create(program: Vec<i64>, inputs: Vec<i64>) -> IntcodeComputer {
        IntcodeComputer {
            program: RefCell::new(program.clone()),
            inputs: RefCell::new(inputs.clone()),
            outputs: RefCell::new(Vec::new()),
            vspace: RefCell::new(HashMap::new()),
            pos: 0,
            relative_base: 0,
            halted: false,
        }
    }

    pub fn read(&self, address: usize) -> i64 {
        if address < self.program.borrow().len() {
            self.program.borrow()[address]
        } else {
            match self.vspace.borrow().get(&address) {
                Some(v) => *v,
                None => 0_i64,
            }
        }
    }

    pub fn write(&mut self, address: usize, value: i64) {
        if address < self.program.borrow().len() {
            self.program.borrow_mut()[address] = value;
        } else {
            self.vspace.borrow_mut().insert(address, value);
        }
    }

    fn param_address(&self, offset: usize, mode: ParamMode) -> usize {
        match mode {
            Imm => self.pos + offset,
            Pos => self.read(self.pos + offset) as usize,
            Rel => (self.read(self.pos + offset) + self.relative_base) as usize,
        }
    }

    fn read_param(&self, offset: usize, mode: ParamMode) -> i64 {
        let addr = self.param_address(offset, mode);
        let v = self.read(addr);
        v
    }

    pub fn process_instruction(&self, instruction: Instruction) -> InstructionEffect {
        match instruction {
            Add(a_mode, b_mode, dest_mode) => {
                let a = self.read_param(1, a_mode);
                let b = self.read_param(2, b_mode);
                let dest = self.param_address(3, dest_mode) as usize;
                WriteMem(dest, a + b)
            }
            Mul(a_mode, b_mode, dest_mode) => {
                let a = self.read_param(1, a_mode);
                let b = self.read_param(2, b_mode);
                let dest = self.param_address(3, dest_mode) as usize;
                WriteMem(dest, a * b)
            }
            Input(mode) => {
                let address = self.param_address(1, mode) as usize;
                let mut inputs = self.inputs.borrow_mut();
                if inputs.len() == 0 {
                    panic!("no more input");
                }
                let value = inputs.remove(0);
                WriteMem(address, value)
            }
            Output(mode) => {
                let v = self.read_param(1, mode);
                OutputValue(v)
            }
            JumpIfTrue(cond_mode, dest_mode) => {
                let cond_val = self.read_param(1, cond_mode);
                if cond_val != 0 {
                    Jump(self.read_param(2, dest_mode) as usize)
                } else {
                    NoEffect
                }
            }
            JumpIfFalse(cond_mode, dest_mode) => {
                let cond_val = self.read_param(1, cond_mode);
                if cond_val == 0 {
                    Jump(self.read_param(2, dest_mode) as usize)
                } else {
                    NoEffect
                }
            }
            LessThan(a_mode, b_mode, dest_mode) => {
                let a = self.read_param(1, a_mode);
                let b = self.read_param(2, b_mode);
                let addr = self.param_address(3, dest_mode);
                WriteMem(addr as usize, if a < b { 1 } else { 0 })
            }
            Equals(a_mode, b_mode, dest_mode) => {
                let a = self.read_param(1, a_mode);
                let b = self.read_param(2, b_mode);
                let addr = self.param_address(3, dest_mode);
                WriteMem(addr as usize, if a == b { 1 } else { 0 })
            }
            AdjustRelBase(mode) => {
                let v = self.read_param(1, mode);
                MoveRelBase(v)
            }
            Term => Halt,
        }
    }

    pub fn handle_effect(&mut self, effect: InstructionEffect) {
        match effect {
            NoEffect => {}
            Halt => {
                self.halted = true;
            }
            OutputValue(value) => {
                self.outputs.borrow_mut().push(value);
            }
            Jump(to) => {
                self.pos = to;
            }
            WriteMem(address, value) => {
                self.write(address, value);
            }
            MoveRelBase(new_relative_base) => {
                self.relative_base += new_relative_base;
                if DEBUG {
                    print!(" >> rel_base={}", self.relative_base);
                }
            }
        }
    }

    pub fn run_next_instruction(&mut self) -> InstructionEffect {
        if self.halted {
            panic!("program already halted");
        }
        let instruction = parse_instruction(self.program.borrow()[self.pos]);
        if DEBUG {
            print!("[{}] {:?}", self.pos, instruction);
        }
        let effect = self.process_instruction(instruction);
        if DEBUG {
            print!(" -> {:?}", effect);
        }
        self.pos += instruction_size(instruction);
        self.handle_effect(effect);
        if DEBUG {
            println!();
        }
        effect
    }

    pub fn add_input(&mut self, val: i64) {
        self.inputs.borrow_mut().push(val);
    }

    pub fn run_until_next_output(&mut self) -> Option<i64> {
        while !self.halted {
            match self.run_next_instruction() {
                OutputValue(val) => {
                    return Some(val);
                }
                _ => {}
            }
        }
        None
    }
}

pub fn parse_program(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

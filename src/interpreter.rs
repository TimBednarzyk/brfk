use ::std;
use std::io::Write;
use std::io::Read;

use ::instructions::Instruction;

/// Represents a Brainfuck program.
///
/// A brainfuck program is given 30000 unsigned bytes of data to work with, and
/// has a single pointer to represent which byte is currently being accessed.
/// There is also a storage byte that is only used with the extended instruction
/// set.
pub struct Program
{
  instructions: Vec<Instruction>,
  data: [u8; 30000],
  storage: u8,
  ins_ptr: usize,
  data_ptr: usize,
}

#[allow(dead_code)]
impl Program
{
  /// Makes a Program from a set of instructions.
  pub fn new(instructions: Vec<Instruction>) -> Program
  {
    Program { instructions: instructions, ins_ptr: 0,
              data: [0; 30000], data_ptr: 0, storage: 0 }
  }

  /// Returns whether the program has completed execution.
  pub fn is_done(&self) -> bool
  {
    self.ins_ptr == self.instructions.len()
  }

  /// Returns the instruction that will be run next, if there is one.
  pub fn get_next_ins(&self) -> Option<Instruction>
  {
    if self.ins_ptr == self.instructions.len()
    {
      Option::None
    }
    else
    {
      Option::Some(self.instructions[self.ins_ptr])
    }
  }

  /// Returns the current value in storage.
  pub fn get_stg(&self) -> u8
  {
    self.storage
  }

  /// Returns the current pointer value.
  pub fn get_ptr(&self) -> usize
  {
    self.data_ptr
  }

  /// Returns the current value at the current pointer.
  pub fn get_val(&self) -> u8
  {
    self.data[self.data_ptr]
  }

  /// Returns a reference to the data for the program
  pub fn get_data<'a>(&'a self) -> &'a [u8; 30000]
  {
    &self.data
  }

  /// Runs through the entire program.
  pub fn run(&mut self)
  {
    while self.step() {}
  }

  /// Runs a single instruction for the program.
  /// Returns false if the program has finished.
  pub fn step(&mut self) -> bool
  {
    if self.is_done()
    {
      return false;
    }

    let instruction = self.instructions[self.ins_ptr];
    self.process_instruction(instruction);

    self.ins_ptr += 1;
    self.is_done()
  }

  fn process_instruction(&mut self, instruction: Instruction)
  {
    match instruction
    {
      Instruction::IncPtr =>
        self.data_ptr = if self.data_ptr == 29999 { 0 }
                        else { self.data_ptr + 1 },
      Instruction::DecPtr =>
        self.data_ptr = if self.data_ptr == 0 { 29999 }
                        else { self.data_ptr - 1 }
        ,
      Instruction::IncVal =>
       self.data[self.data_ptr] = self.data[self.data_ptr].overflowing_add(1).0,
      Instruction::DecVal =>
       self.data[self.data_ptr] = self.data[self.data_ptr].overflowing_sub(1).0,
      Instruction::Output =>
      {
        print!("{}", self.data[self.data_ptr] as char);
        std::io::stdout().flush().unwrap();
      },
      Instruction::Input =>
      {
        let mut byte = [0];
        std::io::stdin().read_exact(&mut byte).unwrap();
        self.data[self.data_ptr] = byte[0];
      },
      Instruction::JmpFwd(i) =>
        if self.data[self.data_ptr] == 0 { self.ins_ptr = i },
      Instruction::JmpBk(i) =>
        if self.data[self.data_ptr] != 0 { self.ins_ptr = i },
      // Extended:
      Instruction::Stop => self.ins_ptr = self.instructions.len() - 1,
      Instruction::Store => self.storage = self.data[self.data_ptr],
      Instruction::Retr => self.data[self.data_ptr] = self.storage,
      Instruction::Rshift => self.data[self.data_ptr] >>= 1,
      Instruction::Lshift => self.data[self.data_ptr] <<= 1,
      Instruction::Not => self.data[self.data_ptr] = !self.data[self.data_ptr],
      Instruction::Xor => self.data[self.data_ptr] ^= self.storage,
      Instruction::And => self.data[self.data_ptr] &= self.storage,
      Instruction::Or => self.data[self.data_ptr] |= self.storage,
      // brfk extensions:
    }
  }
}

impl std::fmt::Display for Program
{
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
  {
    write!(fmt, "Pointer: {}\nStorage: {}\nData:",
           self.data_ptr, self.storage).unwrap();
    for (i, val) in self.data.into_iter().enumerate()
    {
      if i % 150 == 0
      {
        write!(fmt, "\n{:>3}", val).unwrap();
      }
      else
      {
        write!(fmt, ", {:>3}", val).unwrap();
      }
    }
    write!(fmt, "\n")
  }
}

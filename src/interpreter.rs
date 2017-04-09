use ::std;
use std::io::Write;
use std::io::Read;
use std::ops::*;
use ::cell::*;
use ::Language;
use ::instruction::*;

/// A Brainfuck program
pub struct Program
{
  data: Vec<Cell>,
  ins_ptr: usize,
  data_ptr: usize,
  storage_ptr: usize,
  ins_end: usize, // Used only for Basic/SelfModifying
  language: Language,
  done: bool,
}

#[allow(dead_code)]
impl Program
{
  /// Makes a Program from a set of instructions.
  pub fn new(source: &str, language: Language) -> Program
  {
    let mut prog =
      Program { data: Cell::from_str(source, language <= Language::Extended1),
              ins_ptr: 1, data_ptr: 0, storage_ptr: 0, ins_end: 0,
              language: language, done: false };

    prog.ins_end = prog.data.len();
    prog.data_ptr = prog.data.len();
    prog.data.push(Cell::from_char('\0', false));
  }

  /// Returns whether the program has completed execution.
  pub fn is_done(&self) -> bool
  {
    done
  }

  /// Returns the instruction that will be run next, if there is one.
  pub fn get_next_ins(&self) -> Option<Instruction>
  {
    if done
    {
      Option::None
    }
    else
    {
      Instruction::get_instruction(self.data[self.ins_ptr], self.language);
    }
  }

  /// Returns the current value in storage.
  pub fn get_stg(&self) -> u8
  {
    self.data[self.storage_ptr].get_value()
  }

  /// Returns the current pointer value.
  pub fn get_data_ptr(&self) -> usize
  {
    self.data_ptr
  }

  /// Returns the current value at the current pointer.
  pub fn get_val(&self) -> u8
  {
    self.data[self.data_ptr].get_value()
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
    if self.done
    {
      return false;
    }

    if let ins = Instruction::get_instruction(self.data[self.ins_ptr],
                                              self.language)
    {
      self.process_instruction(instruction);
    }

    self.ins_ptr += 1;

    if self.language <= Language::SelfModifying && self.ins_ptr == self.ins_end
    {
      self.done = true;
    }

    !self.done
  }

  fn process_instruction(&mut self, instruction: Instruction)
  {
    match instruction
    {
      Instruction::IncPtr =>
        self.data_ptr = if self.data_ptr == self.data.len() { 0 }
                        else { self.data_ptr + 1 },
      Instruction::DecPtr =>
        self.data_ptr = if self.data_ptr == 0 { self.data.len() - 1 }
                        else { self.data_ptr - 1 },

      Instruction::IncVal =>
        self.data[self.data_ptr].increment(),
      Instruction::DecVal =>
        self.data[self.data_ptr].decrement(),

      Instruction::Output =>
      {
        print!("{}", self.data[self.data_ptr].get_value() as char);
        std::io::stdout().flush().unwrap();
      },
      Instruction::Input =>
      {
        let mut byte = [0];
        std::io::stdin().read_exact(&mut byte).unwrap_or(());
        self.data[self.data_ptr].set_from_char(byte[0] as char);
      },

      Instruction::JmpFwd =>
        if self.data[self.data_ptr] != 0
        {
          let mut level = 1u32;
          while level != 0
          {
            self.data_ptr -= 1;
            match self.data[self.data_ptr].get_instruction()
            {
              Option::Some(Instruction::JmpBkd) => level += 1,
              Option::Some(Instruction::JmpFwd) => level -= 1,
              _ => (),
            }
          }
        }
      Instruction::JmpBkd =>
      // Extended:
      Instruction::Stop => self.done = true,

      Instruction::Store => self.storage.set(self.data[self.data_ptr]),
      Instruction::Retr => self.data[self.data_ptr].set(self.storage),

      Instruction::Rshift => self.data[self.data_ptr].rshift(),
      Instruction::Lshift => self.data[self.data_ptr].lshift(),

      Instruction::Not => self.data[self.data_ptr].not(),
      Instruction::Xor => self.data[self.data_ptr].xor_to_(self.storage),
      Instruction::And => self.data[self.data_ptr].and_to_(self.storage),
      Instruction::Or => self.data[self.data_ptr].or_to_self(self.storage),
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

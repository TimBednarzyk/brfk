use ::std;
use ::std::ops::IndexMut;

/// Represents a single Brainfuck instruction
///
/// # Basic Instructions:
/// 1. `IncPtr` - Increments the pointer by 1
/// 2. `DecPtr` - Decrements the pointer by 1
/// 3. `IncVal` - Increments the value at the pointer by 1
/// 4. `DecVal` - Decrements the value at the pointer by 1
/// 5. `Output` - Outputs a single ASCII character
/// 6. `Input` - Inputs a single ASCII character
/// 7. `JmpFwd` - Paired with JmpBk, represents while(*ptr) {
/// 8. `JmpBk` - Paired with JmpFwd, represents }
///
/// # Extended Instructions:
/// 9. `Stop` - Stops the program
/// 10. `Store` - Stores the value at the pointer in storage
/// 11. `Retr` - Sets the value at the pointer to the value in storage
/// 12. `Rshift` - Does a right logical shift of the value at the pointer
/// 13. `Lshift` - Does a left logical shift of the value at the pointer
/// 14. `Not` - Does a bitwise not of the value at the pointer
/// 15. `Xor` - Does a bitwise xor with the value at the pointer and storage,
///             saving the result in the value at the pointer
/// 15. `And` - Does a bitwise and with the value at the pointer and storage,
///             saving the result in the value at the pointer
/// 15. `Or` - Does a bitwise or with the value at the pointer and storage,
///             saving the result in the value at the pointer
///
/// # BRFK Extensions:
/// 16. `BkPt` - Represents a breakpoint that will cause
///              `Program::run_until_bkpt()` to stop execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction
{
  IncPtr,
  DecPtr,
  IncVal,
  DecVal,
  Output,
  Input,
  JmpFwd(usize),
  JmpBk(usize),
  // Extended:
  Stop,
  Store,
  Retr,
  Rshift,
  Lshift,
  Not,
  Xor,
  And,
  Or,
  // brfk extensions:
}

impl std::fmt::Display for Instruction
{
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
  {
    write!(fmt, "{}", match *self
           {
             Instruction::IncPtr    => ">",
             Instruction::DecPtr    => "<",
             Instruction::IncVal    => "+",
             Instruction::DecVal    => "-",
             Instruction::Output    => ".",
             Instruction::Input     => ",",
             Instruction::JmpFwd(_) => "[",
             Instruction::JmpBk(_)  => "]",
             // Extended:
             Instruction::Stop      => "@",
             Instruction::Store     => "$",
             Instruction::Retr      => "!",
             Instruction::Rshift    => "}",
             Instruction::Lshift    => "{",
             Instruction::Not       => "~",
             Instruction::Xor       => "^",
             Instruction::And       => "&",
             Instruction::Or        => "|",
             // brfk extensions:
           })
  }
}

/// Parses Brainfuck source code into something the interpreter can understand.
/// Comments in the code are ignored.
///
/// # Args:
/// 1. `src` - The source code to parse
/// 2. `enable_ext` - If true, then parse in extended Brainfuck mode, Type 1
///                   See https://esolangs.org/wiki/Extended_Brainfuck
/// 3. `brfk_ext` - If true, enable brfk only extensions to Brainfuck.
///
/// Returns a Result containing the parsed instructions if the code was valid,
/// or a result containing a &'static str error message describing what was
/// wrong.
pub fn parse_code(src: String, enable_ext: bool, brfk_ext: bool) ->
  Result<Vec<Instruction>, &'static str>
{
  let mut instructions = Vec::new();

  let mut jmp_stack = Vec::new();

  for c in src.chars()
  {
    match (c, enable_ext, brfk_ext)
    {
      ('>', _, _) => instructions.push(Instruction::IncPtr),
      ('<', _, _) => instructions.push(Instruction::DecPtr),
      ('+', _, _) => instructions.push(Instruction::IncVal),
      ('-', _, _) => instructions.push(Instruction::DecVal),
      ('.', _, _) => instructions.push(Instruction::Output),
      (',', _, _) => instructions.push(Instruction::Input),
      ('[', _, _) =>
      {
        jmp_stack.push(instructions.len());
        instructions.push(Instruction::JmpFwd(0));
      },
      (']', _, _) =>
      {
        if jmp_stack.len() == 0
        {
          jmp_stack.push(0);
          break;
        }

        let index = jmp_stack.pop().unwrap();
        *instructions.index_mut(index) = Instruction::JmpFwd(instructions.len());
        instructions.push(Instruction::JmpBk(index));
      }
      // Extended:
      ('@', true, _) => instructions.push(Instruction::Stop),
      ('$', true, _) => instructions.push(Instruction::Store),
      ('!', true, _) => instructions.push(Instruction::Retr),
      ('}', true, _) => instructions.push(Instruction::Rshift),
      ('{', true, _) => instructions.push(Instruction::Lshift),
      ('~', true, _) => instructions.push(Instruction::Not),
      ('^', true, _) => instructions.push(Instruction::Xor),
      ('&', true, _) => instructions.push(Instruction::And),
      ('|', true, _) => instructions.push(Instruction::Or),
      // brfk extensions
      _ => (),
    }
  }

  if jmp_stack.len() != 0
  {
    Result::Err("Could not parse code due to unmatched jumps ([ and ]).")
  }
  else
  {
    Result::Ok(instructions)
  }
}

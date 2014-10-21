
pub type RegisterId = uint;

#[deriving(Show,PartialEq,Eq,Clone)]
pub enum Instruction {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Cmp,
  Neg,
  Not,
  BitNot,
  BitXor,
  BitAnd,
  BitOr,
  Lit(int),
  StParam(RegisterId),
  LdParam(RegisterId),
  StReg(RegisterId),
  LdReg(RegisterId),
  JumpZero(uint),
  Jump(uint)
}

impl Instruction {
  pub fn stack_size_delta(&self ) -> int {
    return match *self {
      Add => -1,
      Sub => -1,
      Mul => -1,
      Div => -1,
      Rem => -1,
      Cmp => -1,
      Neg => 0,
      Not => 0,
      BitNot => 0,
      BitXor => -1,
      BitAnd => -1,
      BitOr => -1,
      Lit(_) => 1,
      StParam(_) => -1,
      LdParam(_) => 1,
      StReg(_) => -1,
      LdReg(_) => 1,
      JumpZero(_) => -1,
      Jump(_) => 0
    }
  }
}

#[deriving(Show,Clone)]
pub struct Routine {
  pub num_parameters: uint,
  pub num_registers: uint,
  pub max_stack_size: Option<uint>,
  pub instructions: Vec<Instruction>  
}

impl Routine {
  pub fn new(num_parameters: uint, num_registers: uint, instructions: Vec<Instruction>) -> Routine {
    return Routine {
      num_parameters: num_parameters,
      num_registers: num_registers,
      instructions: instructions,
      max_stack_size: None
    };
  }
}
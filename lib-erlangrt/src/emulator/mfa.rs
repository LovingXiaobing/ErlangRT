//! Module contains reference structs to external and internal functions.
//! M:F/Arity (external), M:F(Args) (apply style), F/Arity (internal).

use crate::{defs::Arity, emulator::funarity::FunArity, fail::RtResult, term::lterm::*};
use core::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Args<'a> {
  // list of args
  List(LTerm),
  // slice of args array
  Slice(&'a [LTerm]),
}

/// Reference to an M:F(Args) function, ready to be called with arguments.
pub struct ModFunArgs<'a> {
  m: LTerm,
  f: LTerm,
  args: Args<'a>,
}

impl<'a> ModFunArgs<'a> {
  pub fn with_args_list(m: LTerm, f: LTerm, args: LTerm) -> ModFunArgs<'a> {
    ModFunArgs {
      m,
      f,
      args: Args::List(args),
    }
  }

  pub fn get_mfarity(&self) -> RtResult<MFArity> {
    Ok(MFArity {
      m: self.m,
      f: self.f,
      arity: self.get_arity()?,
    })
  }

  pub fn get_arity(&self) -> RtResult<usize> {
    match self.args {
      Args::List(lst) => {
        return cons::list_length(lst);
      }
      Args::Slice(s) => {
        return Ok(s.len());
      }
    }
    // panic!("Can't find length for {:?}", self.args)
  }

  pub fn for_each_arg<T>(&self, mut func: T) -> RtResult<()>
  where
    T: FnMut(LTerm) -> RtResult<()>,
  {
    match self.args {
      Args::List(lst) => {
        // ignore return value of for_each but do not ignore a possible error
        cons::for_each(lst, func)?;
      }
      Args::Slice(s) => {
        for elem in s.iter() {
          func(*elem)?;
        }
      }
    }
    Ok(())
  }
}

#[derive(Debug, Copy, Clone)]
pub struct MFArity {
  pub m: LTerm,
  pub f: LTerm,
  pub arity: Arity,
}

impl MFArity {
  pub fn new(m: LTerm, f: LTerm, arity: Arity) -> MFArity {
    MFArity { m, f, arity }
  }

  pub fn from_slice(lterms: &[LTerm]) -> MFArity {
    MFArity {
      m: lterms[0],
      f: lterms[1],
      arity: lterms[2].get_small_unsigned(),
    }
  }

  pub fn new_from_funarity(m: LTerm, fa: &FunArity) -> MFArity {
    MFArity {
      m,
      f: fa.f,
      arity: fa.arity,
    }
  }

  pub fn get_funarity(&self) -> FunArity {
    FunArity::new(self.f, self.arity)
  }
}

impl fmt::Display for MFArity {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{}/{}", self.m, self.f, self.arity)
  }
}

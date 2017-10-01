//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::sync;
use std::collections::BTreeMap;

use function;
use mfa;
use term::Term;

pub type Ptr = sync::Arc<Module>;
pub type Weak = sync::Weak<Module>;

/// Represents a module with collection of functions. Modules are refcounted
/// and can be freed early if the situation allows.
pub struct Module {
  name: Term,
  /// Map to refcounted functions
  code: BTreeMap<mfa::FunArity, function::Ptr>,
  // TODO: attrs
  // TODO: lit table
}

impl Module {
  pub fn new() -> Ptr {
    sync::Arc::new(Module{
      name: Term::non_value(),
      code: BTreeMap::new(),
    })
  }
}
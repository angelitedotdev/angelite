use std::pin::Pin;

use base::rt::Handle;
use func::{Finished, Return};

pub mod func;
pub mod graph;
pub mod param;
pub mod sequence;

pub type System = Pin<Box<dyn Fn() -> Handle<Result<Return, Finished>>>>;

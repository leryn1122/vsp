use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

use crate::StdError;

#[repr(transparent)]
pub struct MessageError<M>(pub M);

impl<M> Debug for MessageError<M>
where
  M: Display + Debug,
{
  fn fmt(&self, f: &mut Formatter) -> Result {
    Debug::fmt(&self.0, f)
  }
}

impl<M> Display for MessageError<M>
where
  M: Display + Debug,
{
  fn fmt(&self, f: &mut Formatter) -> Result {
    Display::fmt(&self.0, f)
  }
}

impl<M> StdError for MessageError<M> where M: Display + Debug + 'static {}

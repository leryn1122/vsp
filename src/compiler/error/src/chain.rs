#[cfg(not(feature = "no-std"))]
use std::vec;

#[cfg(not(feature = "no-std"))]
pub(crate) use crate::Chain;
use crate::StdError;
#[cfg(feature = "no-std")]
pub(crate) struct Chain<'a> {
  state: ChainState<'a>,
}

impl<'a> Chain<'a> {
  pub fn new(head: &'a (dyn StdError + 'static)) -> Self {
    Self {
      state: ChainState::Linked { next: Some(head) },
    }
  }
}

#[derive(Clone)]
pub(crate) enum ChainState<'a> {
  Linked {
    next: Option<&'a (dyn StdError + 'static)>,
  },
  #[cfg(not(feature = "no-std"))]
  Buffered {
    rest: vec::IntoIter<&'a (dyn StdError + 'static)>,
  },
}

impl<'a> Iterator for Chain<'a> {
  type Item = &'a (dyn StdError + 'static);

  fn next(&mut self) -> Option<Self::Item> {
    match &mut self.state {
      ChainState::Linked { next } => {
        let error = (*next)?;
        *next = error.source();
        Some(error)
      }
      #[cfg(not(feature = "no-std"))]
      ChainState::Buffered { rest } => rest.next(),
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = self.len();
    (len, Some(len))
  }
}

#[cfg(not(feature = "no-std"))]
impl DoubleEndedIterator for Chain<'_> {
  fn next_back(&mut self) -> Option<Self::Item> {
    match &mut self.state {
      ChainState::Linked { mut next } => {
        let mut rest = Vec::new();
        while let Some(cause) = next {
          next = cause.source();
          rest.push(cause);
        }
        let mut rest = rest.into_iter();
        let last = rest.next_back();
        self.state = ChainState::Buffered { rest };
        last
      }
      ChainState::Buffered { rest } => rest.next_back(),
    }
  }
}

impl ExactSizeIterator for Chain<'_> {
  fn len(&self) -> usize {
    match &self.state {
      ChainState::Linked { mut next } => {
        let mut len = 0;
        while let Some(cause) = next {
          next = cause.source();
          len += 1;
        }
        len
      }
      #[cfg(not(feature = "no-std"))]
      ChainState::Buffered { rest } => rest.len(),
    }
  }
}

#[cfg(not(feature = "no-std"))]
impl Default for Chain<'_> {
  fn default() -> Self {
    Self {
      state: ChainState::Buffered {
        rest: Vec::new().into_iter(),
      },
    }
  }
}

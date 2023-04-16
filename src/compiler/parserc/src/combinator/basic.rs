use std::marker::PhantomData;
use std::rc::Rc;

use crate::parser::Parser;

/// Pure functor combinator
#[derive(Debug, Clone, Copy)]
pub struct Pure<F> {
  functor: F,
}

impl<F> Pure<F> {
  pub fn new(functor: F) -> Self {
    Self { functor: functor }
  }
}

impl<S, T, F> Parser<S> for Pure<F>
where
  F: Fn() -> T,
{
  type Target = T;

  fn parse<'a>(&self, _: &mut S) -> Option<Self::Target> {
    Some((self.functor)())
  }
}

/// Failure combinator returns nothing.
#[derive(Debug)]
pub struct Empty<T> {
  _ref: PhantomData<fn() -> Option<T>>,
}

impl<T> Empty<T> {
  pub fn new() -> Self {
    Self { _ref: PhantomData }
  }
}

impl<T> Default for Empty<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Clone for Empty<T> {
  fn clone(&self) -> Self {
    Self::default()
  }
}

impl<T> Copy for Empty<T> {}

impl<S, T> Parser<S> for Empty<T> {
  type Target = T;

  fn parse<'a>(&self, _: &mut S) -> Option<Self::Target> {
    None
  }
}

#[derive(Copy, Clone)]
pub struct Some<P> {
  parser: P,
}

impl<P> Some<P> {
  pub fn new(parser: P) -> Self {
    Self { parser: parser }
  }
}

impl<S, P> Parser<S> for Some<P>
where
  S: Clone,
  P: Parser<S>,
{
  type Target = ();

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target> {
    self.parser.parse(state)?;
    let mut s = state.clone();
    while let Some(_) = self.parser.parse(state) {
      s = state.clone();
    }
    *state = s;
    Some(())
  }
}

/// Optional Combinator
#[derive(Copy, Clone)]
pub struct Optional<P> {
  parser: P,
}

impl<P> Optional<P> {
  pub fn new(parser: P) -> Self {
    Self { parser }
  }
}

impl<S, P> Parser<S> for Optional<P>
where
  S: Clone,
  P: Parser<S>,
{
  type Target = Option<P::Target>;

  fn parse(&self, state: &mut S) -> Option<Self::Target> {
    let s = state.clone();
    let fst = self.parser.parse(state);
    match fst {
      None => {
        *state = s;
        Some(None)
      }
      ok => ok.map(Some),
    }
  }
}

// /// Fixed-point combinator which projects the input to itself.
// pub type FixRc<'a, S, T> =
//   Rc<dyn for<'f> Fn(&'f Fix<'a, S, T>) -> Box<dyn Parser<S, Target = T>> +
// 'a>;
//
// pub struct Fix<'a, S, T> {
//   fix: Rc<dyn for<'f> Fn(&'f Fix<'a, S, T>) -> Box<dyn Parser<S, Target = T>>
// + 'a>, }
//
// impl<'a, S, T> Fix<'a, S, T> {
//   pub fn new<F>(fix: F) -> Self
//   where
//     F: for<'f> Fn(&'f Self) -> Box<dyn Parser<S, Target = T> + 'f> + 'a,
//   {
//     Self { fix: Rc::new(fix) }
//   }
// }
//
// impl<'a, S, T> Clone for Fix<'a, S, T> {
//   fn clone(&self) -> Self {
//     Self {
//       fix: self.fix.clone(),
//     }
//   }
// }
//
// impl<'a, S, T> Parser<S> for Fix<'a, S, T> {
//   type Target = T;
//
//   fn parse(&self, state: &mut S) -> Option<Self::Target> {
//     (self.fix)(self).parse(state)
//   }
// }

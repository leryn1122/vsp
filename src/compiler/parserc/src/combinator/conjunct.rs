use crate::parser::convert::IntoParser;
use crate::parser::Parser;

pub struct Or<L, R> {
  pub(crate) left:  L,
  pub(crate) right: R,
}

impl<L, R> Or<L, R> {
  pub fn new(left: L, right: R) -> Self {
    Self {
      left:  left,
      right: right,
    }
  }
}

impl<L, R, S> Parser<S> for Or<L, R>
where
  L: Parser<S>,
  R: Parser<S, Target = L::Target>,
  S: Clone,
{
  type Target = L::Target;

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target> {
    let temp = state.clone();
    let left = self.left.parse(state);
    if let Some(res) = left {
      return Some(res);
    }
    *state = temp;
    self.right.parse(state)
  }
}

pub struct And<L, R> {
  pub(crate) left:  L,
  pub(crate) right: R,
}

impl<L, R> And<L, R> {
  pub fn new(left: L, right: R) -> Self {
    Self {
      left:  left,
      right: right,
    }
  }
}

impl<L, R, S> Parser<S> for And<L, R>
where
  L: Parser<S>,
  R: Parser<S>,
{
  type Target = R::Target;

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target> {
    self.left.parse(state)?;
    self.right.parse(state)
  }
}

pub struct Map<P, F> {
  pub(crate) parser:  P,
  pub(crate) functor: F,
}

impl<P, F> Map<P, F> {
  pub fn new(parser: P, functor: F) -> Self {
    Self {
      parser:  parser,
      functor: functor,
    }
  }
}

impl<S, B, P, F> Parser<S> for Map<P, F>
where
  P: Parser<S>,
  F: Fn(P::Target) -> B,
{
  type Target = B;

  fn parse(&self, state: &mut S) -> Option<Self::Target> {
    self.parser.parse(state).map(&self.functor)
  }
}

pub struct AndThen<P, F> {
  pub(crate) parser:  P,
  pub(crate) functor: F,
}

impl<P, F> AndThen<P, F> {
  pub fn new(parser: P, functor: F) -> Self {
    Self {
      parser:  parser,
      functor: functor,
    }
  }
}

impl<S, L, R, F> Parser<S> for AndThen<L, F>
where
  L: Parser<S>,
  R: IntoParser<S>,
  F: Fn(L::Target) -> R,
{
  type Target = R::Target;

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target> {
    let s = self.parser.parse(state)?;
    (self.functor)(s).into_parser().parse(state)
  }
}

#[derive(Clone, Copy)]
pub struct Filter<P, F> {
  parser:  P,
  functor: F,
}

impl<P, F> Filter<P, F> {
  pub fn new(parser: P, functor: F) -> Self {
    Self {
      parser:  parser,
      functor: functor,
    }
  }
}

impl<S, P, F> Parser<S> for Filter<P, F>
where
  P: Parser<S>,
  F: Fn(&P::Target) -> bool,
{
  type Target = P::Target;

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target> {
    self.parser.parse(state).filter(&self.functor)
  }
}

#[derive(Copy, Clone)]
pub struct Chain<L, R> {
  left:  L,
  right: R,
}

impl<L, R> Chain<L, R> {
  pub fn new(left: L, right: R) -> Self {
    Self {
      left:  left,
      right: right,
    }
  }
}

impl<S, L, R> Parser<S> for Chain<L, R>
where
  L: Parser<S>,
  R: Parser<S>,
  L::Target: IntoIterator,
  R::Target: IntoIterator<Item = <<L as Parser<S>>::Target as IntoIterator>::Item>,
{
  type Target = std::iter::Chain<
    <<L as Parser<S>>::Target as IntoIterator>::IntoIter,
    <<R as Parser<S>>::Target as IntoIterator>::IntoIter,
  >;

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target> {
    let left = self.left.parse(state)?.into_iter();
    let right = self.right.parse(state)?.into_iter();
    Some(left.chain(right))
  }
}

#[derive(Copy, Clone)]
pub struct Many<P> {
  parser: P,
}

impl<P> Many<P> {
  pub fn new(parser: P) -> Self {
    Self { parser: parser }
  }
}

impl<S, P> Parser<S> for Many<P>
where
  S: Clone,
  P: Parser<S>,
{
  type Target = Vec<P::Target>;

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target> {
    let mut vec = vec![];
    let mut s = state.clone();
    while let Some(t) = self.parser.parse(state) {
      vec.push(t);
      s = state.clone();
    }
    *state = s;
    Some(vec)
  }
}

/// Multiple Choice Combinator
pub struct Choice<S, A> {
  parsers: Vec<Box<dyn Parser<S, Target = A>>>,
}

impl<S, A> Choice<S, A> {
  pub fn new(parsers: Vec<Box<dyn Parser<S, Target = A>>>) -> Self {
    Self { parsers: parsers }
  }
}

pub fn choice<S, A>(parsers: Vec<Box<dyn Parser<S, Target = A>>>) -> Choice<S, A>
where
  S: Clone,
{
  Choice::new(parsers)
}

impl<S: Clone, A> Parser<S> for Choice<S, A>
where
  S: Clone,
{
  type Target = A;

  fn parse(&self, state: &mut S) -> Option<Self::Target> {
    for p in self.parsers.iter() {
      let s = state.clone();
      if let ok @ Some(_) = p.parse(state) {
        return ok;
      }
      *state = s;
    }
    None
  }
}

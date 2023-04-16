//
// pub struct And<L, R> {
//   pub(crate) left:  L,
//   pub(crate) right: R,
// }
//
// impl<L, R> Parser for And<L, R>
// where
//   L: Parser,
//   R: Parser,
// {
//   type Target = R::Target;
//
//   fn parse<'a>(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
//     self.left.parse(state).unwrap();
//     self.right.parse(state)
//   }
// }
//
// pub struct Or<L, R> {
//   pub(crate) left:  L,
//   pub(crate) right: R,
// }
//
// impl<L, R> Parser for Or<L, R>
// where
//   L: Parser,
//   R: Parser<Target = L::Target>,
// {
//   type Target = L::Target;
//
//   fn parse<'a>(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
//     let tmp = state.clone();
//     let left = self.left.parse(state);
//     if let Some(res) = left {
//       return Some(res);
//     }
//     *state = tmp;
//     self.right.parse(state)
//   }
// }
//
// pub struct Map<P, F> {
//   pub(crate) parser:  P,
//   pub(crate) functor: F,
// }
//
// impl<B, P, F> Parser for Map<P, F>
// where
//   P: Parser,
//   F: Fn(P::Target) -> B,
// {
//   type Target = B;
//
//   fn parse<'a>(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
//     self.parser.parse(state).map(&self.functor)
//   }
// }

// pub struct AndThen<P, F> {
//   pub(crate) parser:  P,
//   pub(crate) functor: F,
// }
//
// impl<L, R, F> Parser for AndThen<L, F>
// where
//   L: Parser,
//   R: Parser,
//   F: Fn(L::Target) -> R,
// {
//   type Target = R::Target;
//
//   fn parse<'a>(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
//     let f = (self.functor).parse(state).unwrap();
//     let x = self.parser.parse(state).unwrap();
//     Some(f(x))
//   }
// }

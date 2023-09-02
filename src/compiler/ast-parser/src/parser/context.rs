// use std::iter::Peekable;
// use std::ops::Deref;
// use std::rc::Rc;
// use std::slice::Iter;
//
// use crate::parser::token::LocatableToken;
// use crate::parser::token::TokenStream;
//
// pub(crate) type TokenIterator<'a> = Peekable<Iter<'a, LocatableToken>>;
//
// ///
// pub struct TokenContext<'a> {
//   tokens:   Rc<TokenStream>,
//   location: usize,
//   current:  Option<&'a LocatableToken>,
//   prev:     Option<&'a LocatableToken>,
// }
//
// impl<'a> TokenContext<'a> {
//   pub fn from(tokens: &'a TokenStream) -> Self {
//     Self {
//       tokens:   Rc::new(tokens.clone()),
//       location: 0,
//       current:  None,
//       prev:     None,
//     }
//   }
//
//   #[inline]
//   pub fn location(&self) -> usize {
//     self.location.clone()
//   }
//
//   #[inline]
//   pub fn current(&mut self) -> Option<&'a LocatableToken> {
//     self.current
//   }
//
//   #[inline]
//   pub unsafe fn current_unchecked(&mut self) -> &'a LocatableToken {
//     self.current.unwrap_unchecked()
//   }
//
//   pub fn get(&mut self) -> Option<&'a LocatableToken> {
//     self.next()
//   }
//
//   pub fn peek(&self) -> Option<&'a LocatableToken> {
//     self.tokens.get(&self.location + 1)
//   }
//
//   pub fn prev(&self) -> Option<&'a LocatableToken> {
//     self.prev
//   }
// }
//
// impl<'a> Iterator for TokenContext<'a> {
//   type Item = &'a LocatableToken;
//
//   fn next(&mut self) -> Option<Self::Item> {
//     let res = self.tokens.deref().get(self.location.clone() + 1);
//     if res.is_some() {
//       self.location += 1;
//       self.prev = self.current;
//       self.current = res;
//     }
//     self.current
//   }
// }

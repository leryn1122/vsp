use std::cell::Ref;

pub struct SymTable {}

pub struct Frame<'a> {
  parent: Ref<'a, Frame<'a>>,
}

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Pointer;
use std::ops::Deref;
use std::ops::DerefMut;

pub fn make_shared_ptr<T: 'static>(value: T) -> SharedPtr<T> {
  SharedPtr {
    ptr: Box::new(value),
  }
}

pub struct SharedPtr<T: ?Sized> {
  ptr: Box<T>,
}

impl<T> SharedPtr<[T]> {
  #[inline(never)]
  pub fn from_vec(v: Vec<T>) -> SharedPtr<[T]> {
    Self {
      ptr: v.into_boxed_slice(),
    }
  }

  #[inline(never)]
  pub fn into_vec(self) -> Vec<T> {
    self.ptr.into_vec()
  }
}

impl<T> Default for SharedPtr<[T]> {
  fn default() -> SharedPtr<[T]> {
    Self {
      ptr: Box::default(),
    }
  }
}

impl<T: ?Sized> Deref for SharedPtr<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.ptr
  }
}

impl<T: ?Sized> DerefMut for SharedPtr<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.ptr
  }
}

impl<T: 'static + Clone> Clone for SharedPtr<T> {
  fn clone(&self) -> SharedPtr<T> {
    make_shared_ptr((**self).clone())
  }
}

impl<T: ?Sized + Debug> Debug for SharedPtr<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self.ptr, f)
  }
}

impl<T: Display> Display for SharedPtr<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&**self, f)
  }
}

impl<T> Pointer for SharedPtr<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Pointer::fmt(&self.ptr, f)
  }
}

impl<T: Clone> Clone for SharedPtr<[T]> {
  fn clone(&self) -> SharedPtr<[T]> {
    Self::from_vec(self.to_vec())
  }
}

impl<T> From<Vec<T>> for SharedPtr<[T]> {
  fn from(v: Vec<T>) -> Self {
    Self::from_vec(v)
  }
}

impl<T> Into<Vec<T>> for SharedPtr<[T]> {
  fn into(self) -> Vec<T> {
    self.into_vec()
  }
}

impl<T> FromIterator<T> for SharedPtr<[T]> {
  fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> SharedPtr<[T]> {
    Self::from_vec(iter.into_iter().collect())
  }
}

impl<T> IntoIterator for SharedPtr<[T]> {
  type IntoIter = std::vec::IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    self.into_vec().into_iter()
  }
}

impl<'a, T> IntoIterator for &'a SharedPtr<[T]> {
  type IntoIter = std::slice::Iter<'a, T>;
  type Item = &'a T;

  fn into_iter(self) -> Self::IntoIter {
    self.ptr.into_iter()
  }
}

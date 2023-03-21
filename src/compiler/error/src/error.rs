use core::any::TypeId;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use core::mem::ManuallyDrop;

use crate::backtrace::Backtrace;
use crate::ptr::Mut;
use crate::ptr::Own;
use crate::ptr::Ref;
use crate::Chain;
use crate::StdError;
use crate::VspError;

#[repr(C)]
pub(crate) struct ErrorImpl<E = ()> {
  vtable:               &'static ErrorVTable,
  pub(crate) backtrace: Option<Backtrace>,
  // Don't use directly.
  _object:              E,
}

impl<E> ErrorImpl<E> {
  fn erase(&self) -> Ref<ErrorImpl> {
    Ref::new(self).cast::<ErrorImpl>()
  }
}

impl ErrorImpl {
  pub(crate) unsafe fn error(this: Ref<Self>) -> &(dyn StdError + Send + Sync + 'static) {
    (vtable(this.ptr).object_ref)(this).deref()
  }

  #[cfg(not(feature = "no-std"))]
  pub(crate) unsafe fn error_mut(this: Mut<Self>) -> &mut (dyn StdError + Send + Sync + 'static) {
    return (vtable(this.ptr).object_ref)(this.by_ref())
      .by_mut()
      .deref_mut();
  }

  #[cfg(not(feature = "no-std"))]
  #[allow(dead_code)]
  pub(crate) unsafe fn backtrace(this: Ref<Self>) -> &Backtrace {
    this
      .deref()
      .backtrace
      .as_ref()
      .or_else(|| {
        return (vtable(this.ptr).object_backtrace)(this);
      })
      .expect("backtrace capture failed")
  }

  pub(crate) unsafe fn chain(this: Ref<Self>) -> Chain {
    Chain::new(Self::error(this))
  }
}

impl<E> Debug for ErrorImpl<E>
where
  E: Debug,
{
  fn fmt(&self, f: &mut Formatter) -> Result {
    unsafe { ErrorImpl::debug(self.erase(), f) }
  }
}

impl<E> Display for ErrorImpl<E>
where
  E: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result {
    unsafe { Display::fmt(ErrorImpl::error(self.erase()), f) }
  }
}

impl<E> StdError for ErrorImpl<E>
where
  E: StdError,
{
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    unsafe { ErrorImpl::error(self.erase()).source() }
  }
}

//================================================================================================//

unsafe fn vtable(p: core::ptr::NonNull<ErrorImpl>) -> &'static ErrorVTable {
  *(p.as_ptr() as *const &'static ErrorVTable)
}

#[allow(dead_code)]
struct ErrorVTable {
  object_drop:      unsafe fn(Own<ErrorImpl>),
  object_ref:       unsafe fn(Ref<ErrorImpl>) -> Ref<dyn StdError + Send + Sync + 'static>,
  object_boxed:     unsafe fn(Own<ErrorImpl>) -> Box<dyn StdError + Send + Sync + 'static>,
  object_downcast:  unsafe fn(Ref<ErrorImpl>, TypeId) -> Option<Ref<()>>,
  object_drop_rest: unsafe fn(Own<ErrorImpl>, TypeId),
  object_backtrace: unsafe fn(Ref<ErrorImpl>) -> Option<&Backtrace>,
}

unsafe fn object_drop<E>(e: Own<ErrorImpl>) {
  let unerased = e.cast::<ErrorImpl<E>>().boxed();
  drop(unerased);
}

unsafe fn object_drop_front<E>(e: Own<ErrorImpl>, target: TypeId) {
  let _ = target;
  let unerased = e.cast::<ErrorImpl<ManuallyDrop<E>>>().boxed();
  drop(unerased);
}

unsafe fn object_ref<E>(e: Ref<ErrorImpl>) -> Ref<dyn StdError + Send + Sync + 'static>
where
  E: StdError + Send + Sync + 'static,
{
  let unerased = e.cast::<ErrorImpl<E>>();
  return Ref::from_raw(core::ptr::NonNull::new_unchecked(core::ptr::addr_of!(
    (*unerased.as_ptr())._object
  ) as *mut E));
}

#[allow(dead_code)]
unsafe fn object_mut<E>(e: Mut<ErrorImpl>) -> &mut (dyn StdError + Send + Sync + 'static)
where
  E: StdError + Send + Sync + 'static,
{
  &mut e.cast::<ErrorImpl<E>>().deref_mut()._object
}

unsafe fn object_boxed<E>(e: Own<ErrorImpl>) -> Box<dyn StdError + Send + Sync + 'static>
where
  E: StdError + Send + Sync + 'static,
{
  e.cast::<ErrorImpl<E>>().boxed()
}

unsafe fn object_downcast<E>(e: Ref<ErrorImpl>, target: TypeId) -> Option<Ref<()>>
where
  E: 'static,
{
  if TypeId::of::<E>() == target {
    let unerased = e.cast::<ErrorImpl<E>>();
    return Some(
      Ref::from_raw(core::ptr::NonNull::new_unchecked(core::ptr::addr_of!(
        (*unerased.as_ptr())._object
      ) as *mut E))
      .cast::<()>(),
    );
  } else {
    None
  }
}

fn no_backtrace(e: Ref<ErrorImpl>) -> Option<&Backtrace> {
  let _ = e;
  None
}

//================================================================================================//

// #[cfg(not(feature = "no-std"))]
// impl<E> From<E> for VspError
// where
//   E: StdError + Send + Sync + 'static,
// {
//   fn from(error: E) -> Self {
//     let backtrace = backtrace_if_absent!(&error);
//     VspError::from_std(error, backtrace)
//   }
// }

impl VspError {
  #[must_use]
  #[cfg(not(feature = "no-std"))]
  pub fn new<E>(error: E) -> Self
  where
    E: StdError + Send + Sync + 'static,
  {
    let backtrace = backtrace_if_absent!(&error);
    Self::from_std(error, backtrace)
  }

  #[must_use]
  pub fn msg<M>(message: M) -> Self
  where
    M: Display + Debug + Send + Sync + 'static,
  {
    Self::from_adhoc(message, backtrace!())
  }

  #[cfg(not(feature = "no-std"))]
  pub(crate) fn from_std<E>(error: E, backtrace: Option<Backtrace>) -> Self
  where
    E: StdError + Send + Sync + 'static,
  {
    let vtable = &ErrorVTable {
      object_drop:      object_drop::<E>,
      object_ref:       object_ref::<E>,
      object_boxed:     object_boxed::<E>,
      object_downcast:  object_downcast::<E>,
      object_drop_rest: object_drop_front::<E>,
      object_backtrace: no_backtrace,
    };
    unsafe { VspError::construct(error, vtable, backtrace) }
  }

  pub(crate) fn from_adhoc<M>(message: M, backtrace: Option<Backtrace>) -> Self
  where
    M: Display + Debug + Send + Sync + 'static,
  {
    use crate::wrapper::MessageError;
    let error: MessageError<M> = MessageError(message);
    let vtable = &ErrorVTable {
      object_drop:      object_drop::<MessageError<M>>,
      object_ref:       object_ref::<MessageError<M>>,
      object_boxed:     object_boxed::<MessageError<M>>,
      object_downcast:  object_downcast::<M>,
      object_drop_rest: object_drop_front::<M>,
      object_backtrace: no_backtrace,
    };
    unsafe { VspError::construct(error, vtable, backtrace) }
  }

  unsafe fn construct<E>(
    error: E,
    vtable: &'static ErrorVTable,
    backtrace: Option<Backtrace>,
  ) -> Self
  where
    E: StdError + Send + Sync + 'static,
  {
    let ptr: Box<ErrorImpl<E>> = Box::new(ErrorImpl {
      vtable,
      backtrace,
      _object: error,
    });
    let ptr = Own::new(ptr).cast::<ErrorImpl>();
    Self { ptr }
  }

  /// If the error is of given type.
  pub fn is<E>(&self) -> bool
  where
    E: Display + Send + Sync + 'static,
  {
    self.downcast_ref::<E>().is_some()
  }

  /// Downcast the error to the error of specified type.
  pub fn downcast<E>(mut self) -> core::result::Result<E, Self>
  where
    E: Display + Send + Sync + 'static,
  {
    let target = TypeId::of::<E>();
    let inner = self.ptr.by_mut();
    unsafe {
      let addr = match (vtable(inner.ptr).object_downcast)(inner.by_ref(), target) {
        Some(addr) => addr.by_mut().extend(),
        None => return Err(self),
      };
      let outer = ManuallyDrop::new(self);
      let error = addr.cast::<E>().read();
      (vtable(outer.ptr.ptr).object_drop_rest)(outer.ptr, target);
      Ok(error)
    }
  }

  pub fn downcast_ref<E>(&self) -> Option<&E>
  where
    E: Display + Send + Sync + 'static,
  {
    let target = TypeId::of::<E>();
    unsafe {
      let addr = (vtable(self.ptr.ptr).object_downcast)(self.ptr.by_ref(), target)?;
      Some(addr.cast::<E>().deref())
    }
  }

  pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
  where
    E: Display + Send + Sync + 'static,
  {
    let target = TypeId::of::<E>();
    unsafe {
      #[cfg(not(anyhow_no_ptr_addr_of))]
      let addr = (vtable(self.ptr.ptr).object_downcast)(self.ptr.by_ref(), target)?.by_mut();
      Some(addr.cast::<E>().deref_mut())
    }
  }
}

#[cfg(not(feature = "no-std"))]
impl core::ops::Deref for VspError {
  type Target = dyn StdError + Send + Sync + 'static;

  fn deref(&self) -> &Self::Target {
    unsafe { ErrorImpl::error(self.ptr.by_ref()) }
  }
}

#[cfg(not(feature = "no-std"))]
impl core::ops::DerefMut for VspError {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { ErrorImpl::error_mut(self.ptr.by_mut()) }
  }
}

impl Display for VspError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    unsafe { ErrorImpl::display(self.ptr.by_ref(), formatter) }
  }
}

impl Debug for VspError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    unsafe { ErrorImpl::debug(self.ptr.by_ref(), formatter) }
  }
}

impl StdError for VspError {}

impl core::ops::Drop for VspError {
  fn drop(&mut self) {
    unsafe {
      (vtable(self.ptr.ptr).object_drop)(self.ptr);
    }
  }
}

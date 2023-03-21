use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use core::fmt::Write;

use crate::ptr::Ref;
use crate::Chain;
use crate::ErrorImpl;

impl ErrorImpl {
  pub(crate) unsafe fn display(this: Ref<Self>, f: &mut Formatter) -> Result {
    write!(f, "{}", Self::error(this))?;

    if f.alternate() {
      for cause in Self::chain(this).skip(1) {
        write!(f, ": {}", cause)?;
      }
    }

    Ok(())
  }

  pub(crate) unsafe fn debug(this: Ref<Self>, f: &mut Formatter<'_>) -> Result {
    let error = Self::error(this);

    if f.alternate() {
      return Debug::fmt(error, f);
    }

    write!(f, "{}", error)?;

    if let Some(cause) = error.source() {
      write!(f, "\n\nCaused by:")?;
      let multiple = cause.source().is_some();
      for (n, error) in Chain::new(cause).enumerate() {
        writeln!(f)?;
        let mut indented = Indented {
          inner:   f,
          number:  if multiple { Some(n) } else { None },
          started: false,
        };
        write!(indented, "{}", error)?;
      }
    }
    Ok(())
  }
}

struct Indented<'a, D> {
  inner:   &'a mut D,
  number:  Option<usize>,
  started: bool,
}

impl<T> Write for Indented<'_, T>
where
  T: Write,
{
  fn write_str(&mut self, s: &str) -> Result {
    for (i, line) in s.split('\n').enumerate() {
      if !self.started {
        self.started = true;
        match self.number {
          Some(number) => write!(self.inner, "{: >5}: ", number)?,
          None => self.inner.write_str("    ")?,
        }
      } else if i > 0 {
        self.inner.write_char('\n')?;
        if self.number.is_some() {
          self.inner.write_str("       ")?;
        } else {
          self.inner.write_str("    ")?;
        }
      }

      self.inner.write_str(line)?;
    }

    Ok(())
  }
}

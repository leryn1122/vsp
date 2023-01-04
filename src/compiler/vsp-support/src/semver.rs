use std::fmt::{Display, Formatter};
use std::ptr::NonNull;
use std::str::FromStr;

/// # [Semantic Versioning](https://semver.org/)
///
/// ```rust
/// use support::semver::Version;
///
/// let version = Version::new(0, 1, 0);
/// ```
pub struct Version {
  /// Major release
  pub(crate) major: u8,
  /// Minor release
  pub(crate) minor: u8,
  /// Patch release
  pub(crate) patch: u8,
  /// Pre release
  pub(crate) pre_release: Option<PreRelease>,
  /// Build metadata
  pub(crate) build_metadata: Option<BuildMetadata>,
}

impl Version {
  /// Construct a new semantic version.
  pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
    Self {
      major,
      minor,
      patch,
      pre_release: None,
      build_metadata: None,
    }
  }

  /// Construct a default semantic version for new project.
  pub const fn default() -> Self {
    Self::new(0, 1, 0)
  }

  pub fn parse(s: impl Into<String>) -> anyhow::Result<Self> {
    let str = s.into();
    let str = str.as_str();
    Self::from_str(str)
  }
}

impl FromStr for Version {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    todo!()
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}

#[derive(PartialEq)]
pub(crate) enum PreRelease {
  None,
  Alpha,
  Beta,
  ReleaseCandidate,
}

pub(crate) struct BuildMetadata {}

const PTR_BYTES: usize = std::mem::size_of::<NonNull<u8>>();
const TAIL_BYTES: usize = 8 * (PTR_BYTES < 8) as usize - PTR_BYTES * (PTR_BYTES < 8) as usize;

pub(crate) struct Identifier {
  head: NonNull<u8>,
  tail: [u8; TAIL_BYTES],
}

impl Identifier {
  pub(crate) const fn empty() -> Self {
    const HEAD: NonNull<u8> = unsafe { NonNull::new_unchecked(!0 as *mut u8) };
    Self {
      head: HEAD,
      tail: [!0; TAIL_BYTES],
    }
  }
}

pub struct VersionRequisition {
  pub(crate) op: Op,
  pub(crate) requisition: Version,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Op {
  Eq,
  Greater,
  GreaterEq,
  Less,
  LessEq,
}

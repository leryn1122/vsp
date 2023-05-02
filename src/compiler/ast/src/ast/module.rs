/// Module system
///
///
/// ```plaintext
/// ```
use crate::node::NodeId;

///
#[allow(dead_code)]
pub struct Path {
  segments: Vec<PathSegment>,
}

#[allow(dead_code)]
pub struct PathSegment {
  pub id: NodeId,
}

/// <h1>Package</h1>
#[allow(dead_code)]
pub struct Package {
  pub id: NodeId,
}

/// <h1>Module</h1>
#[allow(dead_code)]
pub struct Module {
  pub id: NodeId,
}

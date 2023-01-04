use crate::node::NodeId;

/// Module system
///
///
/// ```plaintext
///
/// ```

///
pub struct Path {
  segments: Vec<PathSegment>,
}

pub struct PathSegment {
  pub id: NodeId,
}

/// <h1>Package</h1>
pub struct Package {
  pub id: NodeId,
}

/// <h1>Module</h1>
pub struct Module {
  pub id: NodeId,
}

// pub const MODULE_NODE_ID: NodeId = NodeId::from_u32(0);

// pub const DUMMY_NODE_ID: NodeId = NodeId::from_u32(u32::MAX);

/// AST Node ID
#[derive(Debug)]
pub struct NodeId {
  id: u32,
}

impl NodeId {
  pub fn from_u32(id: u32) -> Self {
    Self { id }
  }
}

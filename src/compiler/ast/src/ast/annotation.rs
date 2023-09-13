use std::collections::HashMap;

/// # Annotations
///
/// ```vsp
/// @Inline
///
/// @Allocator(Global)
/// ```
pub struct Annotation {
  name:       String,
  attributes: HashMap<String, AnnotationAttribute>,
}

pub type AnnotationAttribute = ();

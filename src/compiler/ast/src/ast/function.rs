use vsp_support::ptr::SharedPtr;

use crate::ast::annotation::Annotation;
use crate::ast::modifier::Accessibility;
use crate::ast::modifier::Constancy;
use crate::ast::stmt::StatementBlock;
use crate::ast::types::Parameter;
use crate::ast::types::Type;

// Aliases for types.
pub type FunctionAccessibility = Accessibility;
pub type FunctionConstancy = Constancy;

/// # Function
/// The function AST represents a function,
///
/// ```vsp
/// public func main() {
///   return 0;
/// }
/// ```
pub struct Function {
  /** Function name */
  pub name: String,
  /** Annotations on the function */
  pub annotations: Option<Vec<Annotation>>,
  /** Function signature */
  pub signature: FunctionSignature,
  /** Function body (statements) */
  pub body: Option<SharedPtr<StatementBlock>>,
}

impl Function {
  pub fn new(name: String, signature: FunctionSignature) -> Self {
    Self {
      name,
      annotations: None,
      signature,
      body: None,
    }
  }
}

/// Function signature or function declarator.
pub struct FunctionSignature {
  /** Function accessibility */
  pub accessibility: FunctionAccessibility,
  /** Function constancy */
  pub constancy: FunctionConstancy,
  /** Parameter list */
  pub parameters: Vec<Parameter>,
  /** Return type */
  pub return_type: Type,
}

impl FunctionSignature {
  pub fn new(
    accessibility: FunctionAccessibility,
    constancy: FunctionConstancy,
    parameters: Vec<Parameter>,
    return_type: Type,
  ) -> Self {
    Self {
      accessibility,
      constancy,
      parameters,
      return_type,
    }
  }
}

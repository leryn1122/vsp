use vsp_support::ptr::SharedPtr;

use crate::ast::annotation::Annotation;
use crate::ast::types::Parameter;
use crate::ast::types::Type;
use crate::ast::Accessibility;
use crate::ast::Constancy;
use crate::ast::StatementBlock;

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
  pub name:        String,
  /** Annotations on the function */
  pub annotations: Option<Vec<Annotation>>,
  /** Function signature */
  pub signature:   FunctionSignature,
  /** Function body (statements) */
  pub body:        Option<SharedPtr<StatementBlock>>,
}

impl Function {
  pub fn new(name: String, signature: FunctionSignature) -> Self {
    Self {
      name:        name,
      annotations: None,
      signature:   signature,
      body:        None,
    }
  }
}

/// Function signature or function declarator.
pub struct FunctionSignature {
  /** Function accessibility */
  pub accessibility: FunctionAccessibility,
  /** Function constancy */
  pub constancy:     FunctionConstancy,
  /** Parameter list */
  pub parameters:    Vec<Parameter>,
  /** Return type */
  pub return_type:   Type,
}

impl FunctionSignature {
  pub fn new(
    accessibility: FunctionAccessibility,
    constancy: FunctionConstancy,
    parameters: Vec<Parameter>,
    return_value: Type,
  ) -> Self {
    Self {
      accessibility: accessibility,
      constancy:     constancy,
      parameters:    parameters,
      return_type:   return_value,
    }
  }
}

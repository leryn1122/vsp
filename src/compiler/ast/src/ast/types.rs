/// Type system implementation for AST.
use core::hash::Hash;
use core::hash::Hasher;

/// # Type System of LLVM Wrapper
/// There are types that are implemented in the type system of LLVM wrapper.
/// - [Primitive Type](https://llvm.org/2.0/docs/LangRef.html#t_primitive)
/// - [Array Type](https://releases.llvm.org/2.0/docs/LangRef.html#t_array)
/// - [Struct Type](https://releases.llvm.org/2.0/docs/LangRef.html#t_struct)
/// - [Function Type](https://releases.llvm.org/2.0/docs/LangRef.html#t_function)
#[derive(Clone, Eq, PartialEq)]
pub enum Type {
  Primitive(PrimitiveType),
  Array(Box<Type>, usize),
  Struct(StructType),
  Function(FunctionType),
  Pointer(StructType),
}

impl Type {
  #[inline]
  pub fn void() -> Self {
    Self::Primitive(PrimitiveType::Void)
  }

  #[inline]
  pub fn int8() -> Self {
    Self::Primitive(PrimitiveType::Int8)
  }

  #[inline]
  pub fn int16() -> Self {
    Self::Primitive(PrimitiveType::Int16)
  }

  #[inline]
  pub fn int32() -> Self {
    Self::Primitive(PrimitiveType::Int32)
  }

  #[inline]
  pub fn int64() -> Self {
    Self::Primitive(PrimitiveType::Int64)
  }

  #[inline]
  pub fn float64() -> Self {
    Self::Primitive(PrimitiveType::Float64)
  }

  #[inline]
  pub fn double() -> Self {
    Self::Primitive(PrimitiveType::Double64)
  }

  #[inline]
  pub fn array(ty: Type, size: usize) -> Self {
    Self::Array(Box::new(ty), size)
  }
}

impl Hash for Type {
  fn hash<H: Hasher>(&self, state: &mut H) {
    core::mem::discriminant(self).hash(state)
  }
}

#[derive(Clone, PartialEq, Eq)]
pub enum PrimitiveType {
  Void,
  Bool,
  Int8,
  Int16,
  Int32,
  Int64,
  Float64,
  Double64,
  Char,
}

impl PrimitiveType {
  pub fn get_name(&self) -> String {
    match self {
      PrimitiveType::Void => "Void",
      PrimitiveType::Bool => "Bool",
      PrimitiveType::Int8 => "Int8",
      PrimitiveType::Int16 => "Int16",
      PrimitiveType::Int32 => "Int32",
      PrimitiveType::Int64 => "Int64",
      PrimitiveType::Float64 => "Float64",
      PrimitiveType::Double64 => "Double64",
      PrimitiveType::Char => "Char",
    }
    .to_string()
  }

  pub fn from_name(name: &str) -> Option<PrimitiveType> {
    match name {
      "Void" => Some(Self::Void),
      "Bool" => Some(Self::Bool),
      "Int8" => Some(Self::Int8),
      "Int16" => Some(Self::Int16),
      "Int32" => Some(Self::Int32),
      "Int64" => Some(Self::Int64),
      "Float64" => Some(Self::Float64),
      "Double64" => Some(Self::Double64),
      "Char" => Some(Self::Char),
      _ => None,
    }
  }

  pub fn is_bool(&self) -> bool {
    matches!(self, PrimitiveType::Bool)
  }

  pub fn is_int8(&self) -> bool {
    matches!(self, PrimitiveType::Int8)
  }

  pub fn is_int16(&self) -> bool {
    matches!(self, PrimitiveType::Int16)
  }

  pub fn is_int32(&self) -> bool {
    matches!(self, PrimitiveType::Int32)
  }

  pub fn is_int64(&self) -> bool {
    matches!(self, PrimitiveType::Int64)
  }

  pub fn is_float64(&self) -> bool {
    matches!(self, PrimitiveType::Float64)
  }

  pub fn is_char(&self) -> bool {
    matches!(self, PrimitiveType::Char)
  }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FunctionType {
  pub params: Vec<Type>,
  pub ret:    Box<Type>,
}

impl FunctionType {
  pub fn new(params: Vec<Type>, ret: Box<Type>) -> Self {
    Self { params, ret }
  }
}

impl Default for FunctionType {
  fn default() -> Self {
    Self {
      params: vec![],
      ret:    Box::new(Type::void()),
    }
  }
}

#[derive(Clone, PartialEq, Eq)]
pub struct StructType {
  pub record: Vec<(u8, Type)>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ArrayType {
  pub element_type: Box<Type>,
  pub size:         u8,
}

pub struct Parameter;

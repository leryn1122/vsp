/// # LLVM vs. Inkwell
/// This module is using `inkwell`, a type-safe wrapper of expose LLVM APIs.
use std::error::Error;
use std::ffi::c_void;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::values::AnyValue;
use inkwell::OptimizationLevel;

pub mod llvm;
pub(crate) mod types;

/// ```rust
/// use std::collections::HashMap;
///
/// unsafe fn codegen(input: Vec<Expr>) {
///   let context = llvm::core::LLVMContextCreate();
///   let module = llvm::core::LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
///   let builder = llvm::core::LLVMCreateBuilderInContext(context);
///
///   // In LLVM, you get your types from functions.
///   let int_type = llvm::core::LLVMInt64TypeInContext(context);
///   let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
///   let function =
///     llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);
///
///   let entry_name = CString::new("entry").unwrap();
///   let bb = llvm::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
///   llvm::core::LLVMPositionBuilderAtEnd(builder, bb);
///
///   let mut names = HashMap::new();
///   insert_allocations(context, builder, &mut names, &input);
///
///   let int_type = llvm::core::LLVMInt64TypeInContext(context);
///   let zero = llvm::core::LLVMConstInt(int_type, 0, 0);
///
///   let mut return_value = zero; // return value on empty program
///   for expr in input {
///     return_value = codegen_expr(context, builder, function, &mut names, expr);
///   }
///   llvm::core::LLVMBuildRet(builder, return_value);
///
///   // Instead of dumping to stdout, let's write out the IR to `out.ll`
///   let out_file = CString::new("out.ll").unwrap();
///   llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());
///
///   llvm::core::LLVMDisposeBuilder(builder);
///   llvm::core::LLVMDisposeModule(module);
///   llvm::core::LLVMContextDispose(context);
/// }
///
/// unsafe fn insert_allocations(
///   context: LLVMContextRef,
///   builder: LLVMBuilderRef,
///   names: &mut HashMap<String, LLVMValueRef>,
///   exprs: &[Expr],
/// ) {
///   let mut variable_names = HashSet::new();
///   for expr in exprs {
///     match *expr {
///       Expr::Assign(ref name, _) => {
///         variable_names.insert(name);
///       }
///
///       _ => {}
///     }
///   }
///
///   for variable_name in variable_names {
///     let int_type = llvm::core::LLVMInt64TypeInContext(context);
///     let name = CString::new(variable_name.as_bytes()).unwrap();
///     let pointer = llvm::core::LLVMBuildAlloca(builder, int_type, name.as_ptr());
///
///     names.insert(variable_name.to_owned(), pointer);
///   }
/// }
///
/// // When you write out instructions in LLVM, you get back `LLVMValueRef`s. You
/// // can then use these references in other instructions.
/// unsafe fn codegen_expr(
///   context: LLVMContextRef,
///   builder: LLVMBuilderRef,
///   func: LLVMValueRef,
///   names: &mut HashMap<String, LLVMValueRef>,
///   expr: Expr,
/// ) -> LLVMValueRef {
///   match expr {
///     Expr::Literal(int_literal) => {
///       let int_type = llvm::core::LLVMInt64TypeInContext(context);
///       llvm::core::LLVMConstInt(int_type, int_literal.parse().unwrap(), 0)
///     }
///
///     Expr::Add(lhs, rhs) => {
///       let lhs = codegen_expr(context, builder, func, names, *lhs);
///       let rhs = codegen_expr(context, builder, func, names, *rhs);
///
///       let name = CString::new("addtmp").unwrap();
///       llvm::core::LLVMBuildAdd(builder, lhs, rhs, name.as_ptr())
///     }
///
///     Expr::Sub(lhs, rhs) => {
///       let lhs = codegen_expr(context, builder, func, names, *lhs);
///       let rhs = codegen_expr(context, builder, func, names, *rhs);
///
///       let name = CString::new("subtmp").unwrap();
///       llvm::core::LLVMBuildSub(builder, lhs, rhs, name.as_ptr())
///     }
///
///     Expr::Mul(lhs, rhs) => {
///       let lhs = codegen_expr(context, builder, func, names, *lhs);
///       let rhs = codegen_expr(context, builder, func, names, *rhs);
///
///       let name = CString::new("multmp").unwrap();
///       llvm::core::LLVMBuildMul(builder, lhs, rhs, name.as_ptr())
///     }
///
///     Expr::Div(lhs, rhs) => {
///       let lhs = codegen_expr(context, builder, func, names, *lhs);
///       let rhs = codegen_expr(context, builder, func, names, *rhs);
///
///       let name = CString::new("divtmp").unwrap();
///       llvm::core::LLVMBuildUDiv(builder, lhs, rhs, name.as_ptr())
///     }
///
///     Expr::Ref(name) => {
///       let pointer = names.get(&name).unwrap();
///       let name = CString::new(name).unwrap();
///       llvm::core::LLVMBuildLoad(builder, *pointer, name.as_ptr())
///     }
///
///     Expr::Assign(name, expr) => {
///       let new_value = codegen_expr(context, builder, func, names, *expr);
///       let pointer = names.get(&name).unwrap();
///       llvm::core::LLVMBuildStore(builder, new_value, *pointer);
///       new_value
///     }
///
///     Expr::If(condition, then_body, else_body) => {
///       let condition_value = codegen_expr(context, builder, func, names, *condition);
///       let int_type = llvm::core::LLVMInt64TypeInContext(context);
///       let zero = llvm::core::LLVMConstInt(int_type, 0, 0);
///
///       let name = CString::new("is_nonzero").unwrap();
///       let is_nonzero = llvm::core::LLVMBuildICmp(
///         builder,
///         llvm::LLVMIntPredicate::LLVMIntNE,
///         condition_value,
///         zero,
///         name.as_ptr(),
///       );
///
///       let entry_name = CString::new("entry").unwrap();
///       let then_block =
///         llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
///       let else_block =
///         llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
///       let merge_block =
///         llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
///
///       llvm::core::LLVMBuildCondBr(builder, is_nonzero, then_block, else_block);
///
///       llvm::core::LLVMPositionBuilderAtEnd(builder, then_block);
///       let mut then_return = zero;
///       for expr in then_body {
///         then_return = codegen_expr(context, builder, func, names, expr);
///       }
///       llvm::core::LLVMBuildBr(builder, merge_block);
///       let then_block = llvm::core::LLVMGetInsertBlock(builder);
///
///       llvm::core::LLVMPositionBuilderAtEnd(builder, else_block);
///       let mut else_return = zero;
///       for expr in else_body {
///         else_return = codegen_expr(context, builder, func, names, expr);
///       }
///       llvm::core::LLVMBuildBr(builder, merge_block);
///       let else_block = llvm::core::LLVMGetInsertBlock(builder);
///
///       llvm::core::LLVMPositionBuilderAtEnd(builder, merge_block);
///       let phi_name = CString::new("iftmp").unwrap();
///       let phi = llvm::core::LLVMBuildPhi(builder, int_type, phi_name.as_ptr());
///
///       let mut values = vec![then_return, else_return];
///       let mut blocks = vec![then_block, else_block];
///
///       llvm::core::LLVMAddIncoming(phi, values.as_mut_ptr(), blocks.as_mut_ptr(), 2);
///       phi
///     }
///   }
/// }
/// ```
#[allow(dead_code)]
fn nothing() {}

type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;
type VoidFunc = unsafe extern "C" fn() -> c_void;

pub struct CodeGenerator<'ctx> {
  context:          &'ctx Context,
  module:           Module<'ctx>,
  builder:          Builder<'ctx>,
  execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
  fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
    let i64_type = self.context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
    let function = self.module.add_function("sum", fn_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");

    self.builder.position_at_end(basic_block);

    let x = function.get_nth_param(0)?.into_int_value();
    let y = function.get_nth_param(1)?.into_int_value();
    let z = function.get_nth_param(2)?.into_int_value();

    let sum = self.builder.build_int_add(x, y, "sum");
    let sum = self.builder.build_int_add(sum, z, "sum");

    self.builder.build_return(Some(&sum));

    unsafe { self.execution_engine.get_function("sum").ok() }
  }
}

fn main0() -> Result<(), Box<dyn Error>> {
  let context = Context::create();
  let module = context.create_module("sum");
  let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

  let module1 = module.to_owned();
  println!("{}", module1.to_owned().print_to_string().to_string());

  let iter = module1.get_functions();
  iter.for_each(|f| {
    println!("{}", f.print_to_string().to_string());
  });

  let codegen = CodeGenerator {
    context: &context,
    module,
    builder: context.create_builder(),
    execution_engine,
  };

  // println!("{}", module.print_to_string().to_string());

  let sum = codegen.jit_compile_sum().ok_or("Unable to JIT compile `sum`")?;

  let x = 1u64;
  let y = 2u64;
  let z = 3u64;

  unsafe {
    println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
    assert_eq!(sum.call(x, y, z), x + y + z);
  }

  Ok(())
}

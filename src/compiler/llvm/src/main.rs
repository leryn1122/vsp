use std::error::Error;
use std::ffi::c_void;

/// # LLVM vs. Inkwell
/// This module is using `inkwell`, a safe wrapper of expose LLVM APIs.
///
/// TODO: Unwrap the `inkwell`, use `llvm-sys` instead.
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

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
  //
  // fn jit_compile_hello_world(&self) -> Option<JitFunction<VoidFunc>> {
  //     let void_type = self.context.void_type();
  //     let fn_type = void_type.fn_type(&[void_type.into()], false);
  //     let function =
  //     self.module.add_function("hello world", fn_type, None);
  //     let basic_block = self.context.append_basic_block(function, "entry2");
  //     self.builder.position_at_end(basic_block);
  //
  // }
}

fn main() -> Result<(), Box<dyn Error>> {
  let context = Context::create();
  let module = context.create_module("sum");
  let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
  let codegen = CodeGenerator {
    context: &context,
    module,
    builder: context.create_builder(),
    execution_engine,
  };

  let sum = codegen
    .jit_compile_sum()
    .ok_or("Unable to JIT compile `sum`")?;

  let x = 1u64;
  let y = 2u64;
  let z = 3u64;

  unsafe {
    println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
    assert_eq!(sum.call(x, y, z), x + y + z);
  }

  Ok(())
}
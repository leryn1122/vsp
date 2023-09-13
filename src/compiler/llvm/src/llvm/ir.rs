use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::FunctionValue;
use inkwell::values::IntValue;
use vsp_ast::ast::expr::BinaryOp;
use vsp_ast::ast::expr::Expression;

#[allow(dead_code, non_snake_case)]
struct IRBuilder<'ctx> {
  context: &'ctx Context,
  builder: &'ctx Builder<'ctx>,
}

unsafe fn codegen_expr<'ctx>(
  context: &'ctx Context,
  builder: &'ctx Builder,
  func: FunctionValue,
  expr: &Expression,
) -> IntValue<'ctx> {
  match expr {
    Expression::Unit => {}
    Expression::LiteralInteger(int) => {
      let int_type = context.i32_type();
      return int_type.const_int(*int as u64, *int > 0);
    }
    Expression::LiteralFloat(_) => {}
    Expression::LiteralBoolean(_) => {}
    Expression::LiteralString(_) => {}
    Expression::Identifier(_) => {}
    Expression::Unary(_, _) => {}
    Expression::Binary(op, lhs, rhs) => {
      let lhs = codegen_expr(context, builder, func, lhs.as_ref());
      let rhs = codegen_expr(context, builder, func, rhs.as_ref());

      return match op {
        BinaryOp::Add => builder.build_int_add(lhs, rhs, "addtmp"),
        BinaryOp::Subtract => builder.build_int_sub(lhs, rhs, "addtmp"),
        BinaryOp::Multiply => builder.build_int_mul(lhs, rhs, "addtmp"),
        BinaryOp::Division => builder.build_int_unsigned_div(lhs, rhs, "addtmp"),
        _ => unreachable!(),
      };
    }
    Expression::MethodCall(_, _) => {}
    Expression::LambdaExpression(_, _) => {}
  }
  unreachable!();
}

#[cfg(test)]
mod tests {
  use inkwell::context::Context;
  use inkwell::execution_engine::JitFunction;
  use inkwell::OptimizationLevel;

  use super::*;

  #[test]
  pub fn test() {
    unsafe {
      let expr = Expression::Binary(
        BinaryOp::Add,
        Box::new(Expression::LiteralInteger(1)),
        Box::new(Expression::Binary(
          BinaryOp::Multiply,
          Box::new(Expression::LiteralInteger(2)),
          Box::new(Expression::LiteralInteger(5)),
        )),
      );
      let context = Context::create();
      let module = context.create_module("add");
      let builder = context.create_builder();

      let int_type = context.i32_type();
      let fn_type = int_type.fn_type(&[], false);
      let function = module.add_function("add", fn_type, None);
      codegen_expr(&context, &builder, function, &expr);

      let engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
      let func: JitFunction<unsafe extern "C" fn() -> i32> = engine.get_function("add").unwrap();
      println!("{}", func.call());
    }
  }
}

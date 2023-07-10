use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::passes::PassManager;
use inkwell::passes::PassManagerBuilder;
use inkwell::targets::InitializationConfig;
use inkwell::targets::Target;

use vsp_ast::ast::function::Function;

pub mod ir;

pub struct CodegenContext<'ctx> {
  context: &'ctx Context,
  module: Module<'ctx>,
  builder: Builder<'ctx>,
}

impl<'ctx> CodegenContext<'ctx> {
  pub fn new(name: String, context: &'ctx Context) -> Self {
    let module = context.create_module(name.as_str());
    Self {
      context,
      module,
      builder: context.create_builder(),
    }
  }

  pub fn context(&self) -> &'ctx Context {
    self.context
  }

  pub fn module(&self) -> &Module<'ctx> {
    &self.module
  }

  pub fn builder(&self) -> &Builder<'ctx> {
    &self.builder
  }

  pub fn optimize(&mut self) {
    let config = InitializationConfig::default();
    Target::initialize_native(&config).unwrap();
    let pass_manager_builder = PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(OptimizationLevel::Aggressive);
    let mut pass_manager = PassManager::create(());

    // Add passes.
    add_default_pass(&mut pass_manager);
    add_customized_pass(&mut pass_manager);

    pass_manager.run_on(&self.module);
    pass_manager_builder.populate_module_pass_manager(&pass_manager);
  }

  pub fn add_function(&self, function: Function) {
    let i64_type = self.context.i64_type();
    let fn_type = i64_type.fn_type(&[], false);
    let function = self.module.add_function(function.name.as_str(), fn_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");
  }
}

pub fn convert_to_llvm_optimization_level(num: u8) -> () {
  debug_assert!(num <= 3);
}

/// Add default LLVM passes to the pass manager.
#[inline]
fn add_default_pass(manager: &mut PassManager<Module>) {
  manager.add_promote_memory_to_register_pass();
  manager.add_demote_memory_to_register_pass();
  manager.add_argument_promotion_pass();
  manager.add_always_inliner_pass();
  manager.add_gvn_pass();
  manager.add_new_gvn_pass();
  manager.add_function_attrs_pass();
  manager.add_prune_eh_pass();
  manager.add_constant_merge_pass();
  manager.add_scalarizer_pass();
  manager.add_merged_load_store_motion_pass();
  manager.add_instruction_combining_pass();
  manager.add_memcpy_optimize_pass();
  manager.add_partially_inline_lib_calls_pass();
  manager.add_lower_switch_pass();
  manager.add_reassociate_pass();
  manager.add_simplify_lib_calls_pass();
  manager.add_aggressive_inst_combiner_pass();
  manager.add_instruction_simplify_pass();
  manager.add_function_inlining_pass();
  manager.add_global_optimizer_pass();
  manager.add_dead_arg_elimination_pass();
  manager.add_strip_symbol_pass();
  manager.add_strip_dead_prototypes_pass();
  manager.add_internalize_pass(true);
  manager.add_sccp_pass();
  manager.add_aggressive_dce_pass();
  manager.add_global_dce_pass();
  manager.add_tail_call_elimination_pass();
  manager.add_basic_alias_analysis_pass();
  manager.add_licm_pass();
  manager.add_ind_var_simplify_pass();
  manager.add_loop_vectorize_pass();
  manager.add_loop_unswitch_pass();
  manager.add_loop_idiom_pass();
  manager.add_loop_rotate_pass();
  manager.add_loop_unroll_and_jam_pass();
  manager.add_loop_unroll_pass();
  manager.add_loop_deletion_pass();
  manager.add_cfg_simplification_pass();
  manager.add_verifier_pass();
}

/// Add customized LLVM passes to the pass manager.
#[inline]
fn add_customized_pass(_manager: &mut PassManager<Module>) {}

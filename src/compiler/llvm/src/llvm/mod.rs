use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::passes::PassManagerBuilder;
use inkwell::targets::InitializationConfig;
use inkwell::targets::Target;
use inkwell::OptimizationLevel;
use vsp_ast::ast::function::Function;

pub mod ir;

pub struct CodegenContext<'ctx> {
  pub context: &'ctx Context,
  pub module:  Module<'ctx>,
  pub builder: Builder<'ctx>,
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

  pub fn optimize(&mut self) {
    let config = InitializationConfig::default();
    Target::initialize_native(&config).unwrap();
    let pass_manager_builder = PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(OptimizationLevel::Aggressive);
    let pass_manager = PassManager::create(());

    // pass_manager.add_promote_memory_to_register_pass();
    // pass_manager.add_demote_memory_to_register_pass();
    // pass_manager.add_argument_promotion_pass();
    // pass_manager.add_always_inliner_pass();
    // pass_manager.add_gvn_pass();
    // pass_manager.add_new_gvn_pass();
    // pass_manager.add_function_attrs_pass();
    // pass_manager.add_prune_eh_pass();
    // pass_manager.add_constant_merge_pass();
    // pass_manager.add_scalarizer_pass();
    // pass_manager.add_merged_load_store_motion_pass();
    // pass_manager.add_instruction_combining_pass();
    // pass_manager.add_memcpy_optimize_pass();
    // pass_manager.add_partially_inline_lib_calls_pass();
    // pass_manager.add_lower_switch_pass();
    // pass_manager.add_reassociate_pass();
    // pass_manager.add_simplify_lib_calls_pass();
    // pass_manager.add_aggressive_inst_combiner_pass();
    // pass_manager.add_instruction_simplify_pass();
    // pass_manager.add_function_inlining_pass();
    // pass_manager.add_global_optimizer_pass();
    // pass_manager.add_dead_arg_elimination_pass();
    // pass_manager.add_strip_symbol_pass();
    // pass_manager.add_strip_dead_prototypes_pass();
    // pass_manager.add_internalize_pass(true);
    // pass_manager.add_sccp_pass();
    // pass_manager.add_aggressive_dce_pass();
    // pass_manager.add_global_dce_pass();
    // pass_manager.add_tail_call_elimination_pass();
    // pass_manager.add_basic_alias_analysis_pass();
    // pass_manager.add_licm_pass();
    // pass_manager.add_ind_var_simplify_pass();
    // pass_manager.add_loop_vectorize_pass();
    // pass_manager.add_loop_unswitch_pass();
    // pass_manager.add_loop_idiom_pass();
    // pass_manager.add_loop_rotate_pass();
    // pass_manager.add_loop_unroll_and_jam_pass();
    // pass_manager.add_loop_unroll_pass();
    // pass_manager.add_loop_deletion_pass();
    // pass_manager.add_cfg_simplification_pass();
    // pass_manager.add_verifier_pass();

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

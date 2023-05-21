/// Implementation is recommended to be named as `XXXAction`.
pub trait ExecuteAction {}

///
pub struct StatusPrintAction {}

impl ExecuteAction for StatusPrintAction {}

pub struct CompilationInvocation;

#[macro_export]
macro_rules! register_command {
  ($command:ident $name:ident $(,)?) => {
    use crate::ops::$name;
    $command.register($name::cli(), $name::execute);
  };
}

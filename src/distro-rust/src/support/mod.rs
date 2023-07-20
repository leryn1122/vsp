pub mod compression;
pub mod resources;
pub mod shell;

#[macro_export]
macro_rules! debug_println {
  ($($arg:tt)*) => {
    if cfg!(debug_assertions) {
      print!("=======> ");
      println!($($arg)*);
      println!()
    }
  };
}

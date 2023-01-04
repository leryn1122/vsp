use crate::table::Table;

mod table;

pub struct Process {
  pub pid: usize,
  pub name: &'static str,
}

fn main() {
  let mut table = Table::default();
  table
    .add_row(Process {
      pid: 10517,
      name: "java",
    })
    .add_row(Process {
      pid: 4932,
      name: "go",
    });
  table.to_string();
}

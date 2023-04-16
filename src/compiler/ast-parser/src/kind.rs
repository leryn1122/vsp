// pub trait Parser
// where
//   Self: Sized,
// {
// }
//
// pub trait ParserProvider {
//   fn create(&self) -> dyn Parser;
// }
//
// pub struct Traditional;
// impl ParserProvider for Traditional {
//   fn create(&self) -> dyn Parser {
//     let parser = ParserCParser {};
//     parser
//   }
// }
//
// pub struct ParserC;
// impl ParserProvider for ParserC {
//   fn create(&self) -> dyn Parser {
//     let parser = ParserCParser {};
//     parser
//   }
// }
//
// pub struct ParserCParser {}
//
// impl Parser for ParserCParser {}

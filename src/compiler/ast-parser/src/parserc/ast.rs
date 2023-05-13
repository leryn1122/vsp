// use combine::Parser;
// use combine::Stream;
// use vsp_ast::ast::expr::BinaryOpKind;
// use vsp_ast::ast::expr::Expression;
//
// fn expr_parser<Input>() -> impl Parser<Input, Output = Expression>
// where
//   Input: Stream<Token = char>,
// {
//   term_parser().and_then(|lhs| {
//     let op_parser = operator_parser();
//     chain_left(op_parser, term_parser(), || {
//       |rhs| match op_parser.parse(&mut rhs.clone().into_inner()) {
//         Ok(op) => Ok(Expr::Binary(op, Box::new(lhs.clone()), Box::new(rhs))),
//         Err(err) => Err(err),
//       }
//     })
//     .parse_stream(&mut lhs.clone().into_inner())
//     .map_err(|err| err.into_expected())
//   })
// }
//

#[cfg(test)]
mod tests {}

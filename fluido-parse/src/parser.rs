use fluido_types::{concentration::Concentration, error::IRGenerationError, expr::Expr};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mixlang.pest"]
struct MixLangParser;

pub trait Parse
where
    Self: Sized,
{
    fn parse(input_str: &str) -> Result<Self, IRGenerationError>;
}

impl Parse for Expr {
    fn parse(input_str: &str) -> Result<Self, IRGenerationError> {
        let pairs = MixLangParser::parse(Rule::expression, input_str)
            .map_err(|e| IRGenerationError::ParseError(e.to_string()))?;
        build_ast(pairs)
    }
}

fn build_ast(pairs: pest::iterators::Pairs<Rule>) -> Result<Expr, IRGenerationError> {
    let pair = pairs.into_iter().next().unwrap();

    match pair.as_rule() {
        Rule::expression => build_ast(pair.into_inner()),
        Rule::mix => {
            let mut inner_pairs = pair.into_inner();
            let first_expr = build_ast(inner_pairs.next().unwrap().into_inner())?;
            let second_expr = build_ast(inner_pairs.next().unwrap().into_inner())?;
            Ok(Expr::Mix(Box::new(first_expr), Box::new(second_expr)))
        }
        Rule::float => {
            let num = pair.as_str().parse::<f64>().unwrap();
            let concentration = Concentration::from_f64(num);
            Ok(Expr::Number(concentration))
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::Expr;
    use crate::parse::Parse;
    use fluido_types::concentration::Concentration;

    #[test]
    fn pase_single_num() {
        let input_str = "0.2";
        let expr = Expr::parse(input_str).unwrap();
        let expected_expr = Expr::Number(Concentration::from_f64(0.2));
        assert_eq!(expected_expr, expr)
    }

    #[test]
    fn pase_single_mix() {
        let input_str = "(mix 0.2 0.3)";
        let expr = Expr::parse(input_str).unwrap();
        let zero_point_two = Expr::Number(Concentration::from_f64(0.2));
        let zero_point_three = Expr::Number(Concentration::from_f64(0.3));
        let expected_expr = Expr::Mix(Box::new(zero_point_two), Box::new(zero_point_three));
        assert_eq!(expected_expr, expr)
    }

    #[test]
    fn parse_nested_mix() {
        let input_str = "(mix 0.2 (mix 0.3 0.4))";
        let expr = Expr::parse(input_str).unwrap();
        let zero_point_three = Expr::Number(Concentration::from_f64(0.3));
        let zero_point_four = Expr::Number(Concentration::from_f64(0.4));

        let first_mix = Expr::Mix(Box::new(zero_point_three), Box::new(zero_point_four));
        let zero_point_two = Expr::Number(Concentration::from_f64(0.2));
        let expected_expr = Expr::Mix(Box::new(zero_point_two), Box::new(first_mix));

        assert_eq!(expected_expr, expr)
    }
}

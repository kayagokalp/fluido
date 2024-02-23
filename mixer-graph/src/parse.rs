use std::str::FromStr;

use mixer_generator::concentration::Concentration;
use pest::Parser;
use pest_derive::Parser;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Expr {
    Mix(Box<Expr>, Box<Expr>),
    Number(Concentration),
}

#[derive(Parser)]
#[grammar = "mixlang.pest"]
struct MixLangParser;

#[derive(Debug)]
pub struct ParseExprError {
    message: String,
}

impl std::fmt::Display for ParseExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl std::error::Error for ParseExprError {}

impl FromStr for Expr {
    type Err = ParseExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = MixLangParser::parse(Rule::expression, s).map_err(|e| ParseExprError {
            message: e.to_string(),
        })?;
        Ok(build_ast(pairs))
    }
}

fn build_ast(pairs: pest::iterators::Pairs<Rule>) -> Expr {
    let pair = pairs.into_iter().next().unwrap();

    match pair.as_rule() {
        Rule::expression => build_ast(pair.into_inner()),
        Rule::mix => {
            let mut inner_pairs = pair.into_inner();
            let first_expr = build_ast(inner_pairs.next().unwrap().into_inner());
            let second_expr = build_ast(inner_pairs.next().unwrap().into_inner());
            Expr::Mix(Box::new(first_expr), Box::new(second_expr))
        }
        Rule::float => {
            let num = pair.as_str().parse::<f64>().unwrap();
            let concentration = Concentration::from_f64(num);
            Expr::Number(concentration)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use mixer_generator::concentration::Concentration;

    use super::Expr;
    use std::str::FromStr;

    #[test]
    fn pase_single_num() {
        let input_str = "0.2";
        let expr = Expr::from_str(input_str).unwrap();
        let expected_expr = Expr::Number(Concentration::from_f64(0.2));
        assert_eq!(expected_expr, expr)
    }

    #[test]
    fn pase_single_mix() {
        let input_str = "(mix 0.2 0.3)";
        let expr = Expr::from_str(input_str).unwrap();
        let zero_point_two = Expr::Number(Concentration::from_f64(0.2));
        let zero_point_three = Expr::Number(Concentration::from_f64(0.3));
        let expected_expr = Expr::Mix(Box::new(zero_point_two), Box::new(zero_point_three));
        assert_eq!(expected_expr, expr)
    }

    #[test]
    fn parse_nested_mix() {
        let input_str = "(mix 0.2 (mix 0.3 0.4))";
        let expr = Expr::from_str(input_str).unwrap();
        let zero_point_three = Expr::Number(Concentration::from_f64(0.3));
        let zero_point_four = Expr::Number(Concentration::from_f64(0.4));

        let first_mix = Expr::Mix(Box::new(zero_point_three), Box::new(zero_point_four));
        let zero_point_two = Expr::Number(Concentration::from_f64(0.2));
        let expected_expr = Expr::Mix(Box::new(zero_point_two), Box::new(first_mix));

        assert_eq!(expected_expr, expr)
    }
}

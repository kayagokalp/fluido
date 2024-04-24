use fluido_types::{
    concentration::Concentration, error::IRGenerationError, expr::Expr, fluid::Fluid,
};
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
            let concentration = Concentration::from(num);
            Ok(Expr::Concentration(concentration))
        }
        Rule::integer => {
            let num = pair.as_str().parse::<u64>().unwrap();
            Ok(Expr::Vol(num))
        }
        Rule::fluid => {
            let fluid = pair.as_str().parse::<Fluid>().unwrap();
            Ok(Expr::Fluid(fluid))
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parse;
    use fluido_types::{concentration::Concentration, expr::Expr, fluid::Fluid};

    #[test]
    fn parse_fluid() {
        let input_str = "(fluid 0.2 1)";
        let expr = Expr::parse(input_str).unwrap();
        let expected_conc = Concentration::from(0.2);
        let expected_vol = 1;
        let expected_fluid = Expr::Fluid(Fluid::new(expected_conc, expected_vol));
        assert_eq!(expected_fluid, expr)
    }

    #[test]
    fn parse_single_mix() {
        let input_str = "(mix (fluid 0.2 1) (fluid 0.3 1))";
        let expr = Expr::parse(input_str).unwrap();
        let unit_vol = 1u64;

        let zero_point_two = Concentration::from(0.2);
        let zero_point_three = Concentration::from(0.3);
        let first_fluid = Expr::Fluid(Fluid::new(zero_point_two, unit_vol));
        let second_fluid = Expr::Fluid(Fluid::new(zero_point_three, unit_vol));
        let expected_expr = Expr::Mix(Box::new(first_fluid), Box::new(second_fluid));
        assert_eq!(expected_expr, expr)
    }

    #[test]
    fn parse_nested_mix() {
        let input_str = "(mix (fluid 0.2 1) (mix (fluid 0.3 1) (fluid 0.4 1)))";
        let expr = Expr::parse(input_str).unwrap();
        let unit_vol = 1u64;
        let zero_point_two = Concentration::from(0.2);
        let zero_point_three = Concentration::from(0.3);
        let zero_point_four = Concentration::from(0.4);

        let first_fluid = Fluid::new(zero_point_two, unit_vol);
        let second_fluid = Fluid::new(zero_point_three, unit_vol);
        let third_fluid = Fluid::new(zero_point_four, unit_vol);

        let first_fluid_expr = Expr::Fluid(first_fluid);
        let second_fluid_expr = Expr::Fluid(second_fluid);
        let third_fluid_expr = Expr::Fluid(third_fluid);

        let inner_mix = Expr::Mix(Box::new(second_fluid_expr), Box::new(third_fluid_expr));
        let final_mix = Expr::Mix(Box::new(first_fluid_expr), Box::new(inner_mix));

        assert_eq!(final_mix, expr)
    }
}

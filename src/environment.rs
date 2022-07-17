use std::collections::HashMap;

use crate::definitions;

fn parse_list_of_floats(
    args: &[definitions::LustExpression],
) -> Result<Vec<f64>, definitions::LustErrors> {
    args.iter()
        .map(|x| parse_single_float(x))
        .collect::<Result<Vec<f64>, definitions::LustErrors>>()
}

fn parse_single_float(exp: &definitions::LustExpression) -> Result<f64, definitions::LustErrors> {
    match exp {
        definitions::LustExpression::Number(num) => Ok(*num),
        _ => Err(definitions::LustErrors::SyntaxError(
            "expected a number".to_string(),
        )),
    }
}

macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[definitions::LustExpression]| -> Result<definitions::LustExpression, definitions::LustErrors> {
            let floats = parse_list_of_floats(args)?;
            let first = floats.first().ok_or(definitions::LustErrors::TypeError(
                "expected at least one number".to_string(),
            ))?;
            let rest = &floats[1..];
            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            }
            Ok(definitions::LustExpression::Bool(f(first, rest)))
        }
    }};
}

// new default environment
pub fn new() -> definitions::LustEnvironment {
    let mut data: HashMap<String, definitions::LustExpression> = HashMap::new();
    data.insert(
        "+".to_string(),
        definitions::LustExpression::Func(
            |args: &[definitions::LustExpression]| -> Result<definitions::LustExpression, definitions::LustErrors> {
                let sum = parse_list_of_floats(args)?
                    .iter()
                    .fold(0.0, |sum, a| sum + a);

                Ok(definitions::LustExpression::Number(sum))
            },
        ),
    );
    data.insert(
        "-".to_string(),
        definitions::LustExpression::Func(
            |args: &[definitions::LustExpression]| -> Result<definitions::LustExpression, definitions::LustErrors> {
                let floats = parse_list_of_floats(args)?;
                let first = *floats.first().ok_or(definitions::LustErrors::TypeError(
                    "expected at least one number".to_string(),
                ))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

                Ok(definitions::LustExpression::Number(first - sum_of_rest))
            },
        ),
    );
    data.insert(
        "=".to_string(),
        definitions::LustExpression::Func(ensure_tonicity!(|a, b| a == b)),
    );
    data.insert(
        ">".to_string(),
        definitions::LustExpression::Func(ensure_tonicity!(|a, b| a > b)),
    );
    data.insert(
        ">=".to_string(),
        definitions::LustExpression::Func(ensure_tonicity!(|a, b| a >= b)),
    );
    data.insert(
        "<".to_string(),
        definitions::LustExpression::Func(ensure_tonicity!(|a, b| a < b)),
    );
    data.insert(
        "<=".to_string(),
        definitions::LustExpression::Func(ensure_tonicity!(|a, b| a <= b)),
    );

    definitions::LustEnvironment { data }
}

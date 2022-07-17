use crate::definitions;

pub fn eval(
    exp: &definitions::LustExpression,
    env: &mut definitions::LustEnvironment,
) -> Result<definitions::LustExpression, definitions::LustErrors> {
    match exp {
        definitions::LustExpression::Symbol(k) => env
            .data
            .get(k)
            .ok_or(definitions::LustErrors::SyntaxError(format!(
                "unexpected symbol k='{}'",
                k
            )))
            .map(|x| x.clone()),
        definitions::LustExpression::Bool(_a) => Ok(exp.clone()),
        definitions::LustExpression::Number(_a) => Ok(exp.clone()),
        definitions::LustExpression::List(list) => {
            let first_form = list
                .first()
                .ok_or(definitions::LustErrors::SyntaxError(format!(
                    "error: empty list"
                )))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                definitions::LustExpression::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<definitions::LustExpression>, definitions::LustErrors>>();
                    f(&args_eval?)
                }
                _ => Err(definitions::LustErrors::SyntaxError(
                    "first form must be a function".to_string(),
                )),
            }
        }
        definitions::LustExpression::Func(_) => Err(definitions::LustErrors::SyntaxError(
            "unexpected form".to_string(),
        )),
    }
}

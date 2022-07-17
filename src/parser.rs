use crate::definitions;

pub fn parse(
    tokens: Vec<String>,
) -> Result<(definitions::LustExpression, Vec<String>), definitions::LustErrors> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(definitions::LustErrors::SyntaxError(
            "could not get token".to_string(),
        ))?;
    match &token[..] {
        // first level parsing
        "(" => read_seq(rest.to_vec()),
        ")" => Err(definitions::LustErrors::UnbalancedParens(format!(
            "unexpected `)` in {:?}",
            tokens
        ))),
        // _ => Ok((parse_atom(token), rest.to_vec())),
        _ => {
            // Symbol parsing
            let res = match token.as_ref() {
                "true" => definitions::LustExpression::Bool(true),
                "false" => definitions::LustExpression::Bool(false),
                _ => match token.parse() {
                    Ok(v) => definitions::LustExpression::Number(v),
                    Err(_) => definitions::LustExpression::Symbol(token.to_string().clone()),
                },
            };
            Ok((res, rest.to_vec()))
        }
    }
}

fn read_seq(
    tokens: Vec<String>,
) -> Result<(definitions::LustExpression, Vec<String>), definitions::LustErrors> {
    let mut res: Vec<definitions::LustExpression> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) =
            xs.split_first()
                .ok_or(definitions::LustErrors::UnbalancedParens(
                    "could not find closing `)`".to_string(),
                ))?;
        if next_token == ")" {
            return Ok((definitions::LustExpression::List(res), rest.to_vec())); // closing `)`, head to the token after
        }
        let (exp, new_xs) = parse(xs.clone().to_vec())?;
        res.push(exp);
        xs = new_xs;
    }
}

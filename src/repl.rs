use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::definitions;
use crate::environment;
use crate::evaluator;
use crate::parser;
use crate::tokenizer;

fn parse_and_evaluate(
    expr: String,
    env: &mut definitions::LustEnvironment,
) -> Result<definitions::LustExpression, definitions::LustErrors> {
    let (parsed_exp, _) = parser::parse(tokenizer::tokenize(expr))?;
    let evaled_exp = evaluator::eval(&parsed_exp, env)?;

    Ok(evaled_exp)
}

pub fn new() {
    let mut rl = Editor::<()>::new();
    let env = &mut environment::new();
    if let Err(_) = rl.load_history("lust_history.txt") {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("=> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                match parse_and_evaluate(line, env) {
                    Ok(res) => println!("=> {}", res),
                    Err(e) => println!("(error) => {:?}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("lust_history.txt").unwrap();
}

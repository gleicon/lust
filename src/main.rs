mod definitions;
mod environment;
mod evaluator;
mod parser;
mod repl;
mod tokenizer;

// repl -> parse(tokens) -> evaluate(env, expressions)
fn main() {
    println!("Lust - lisp in rust");
    repl::new();
}

// http://norvig.com/lispy.html
// https://stopa.io/post/222
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum LustExpression {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<LustExpression>),
    Func(fn(&[LustExpression]) -> Result<LustExpression, LustErrors>),
}

impl fmt::Display for LustExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            LustExpression::Bool(a) => a.to_string(),
            LustExpression::Symbol(s) => s.clone(),
            LustExpression::Number(n) => n.to_string(),
            LustExpression::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            LustExpression::Func(_) => "Function {}".to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub enum LustErrors {
    UnbalancedParens(String),
    SyntaxError(String),
    TypeError(String),
}

#[derive(Clone)]
pub struct LustEnvironment {
    pub data: HashMap<String, LustExpression>,
}

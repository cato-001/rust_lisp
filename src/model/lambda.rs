use std::fmt::{Debug, Display};

use super::{Frame, Symbol, Value};

/// A Lisp function defined in Lisp.
#[derive(Debug, PartialEq, Hash)]
pub struct Lambda {
    pub closure: Frame,
    pub argnames: Vec<Symbol>,
    pub body: Box<Value>,
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body_str = format!("{}", &self.body);
        let argnames = self
            .argnames
            .iter()
            .map(|sym| sym.0.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        write!(f, "({}) {}", argnames, body_str)
    }
}

pub mod error;
pub mod file;
pub mod time;

use regex::Regex;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Filter {
    reg: Regex,
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.reg.fmt(f)
    }
}

impl Filter {
    #[allow(dead_code)]
    pub fn new(expr: &str) -> Result<Filter, String> {
        match Regex::new(expr) {
            Ok(reg) => Ok(Filter { reg }),
            Err(err) => Err(err.to_string()),
        }
    }

    #[allow(dead_code)]
    pub fn is_match(&self, s: &str) -> bool {
        self.reg.is_match(s)
    }
}

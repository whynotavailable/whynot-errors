use std::fmt::Display;

#[derive(Debug)]
pub struct SetupError {
    pub msg: String,
}

impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Setup Error: {}", self.msg)
    }
}

impl SetupError {
    pub fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

pub type SetupResult = Result<(), SetupError>;

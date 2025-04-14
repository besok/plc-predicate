use std::fmt::format;

type FBDResult<T> = Result<T, FBDError>;

trait Defined {}

#[derive(Debug)]
pub enum FBDError {
    InputError(String),
    ExecutionError(String),
    ConfigurationError(String),
    StateError(String),
    CriticalError(String),
}

impl FBDError {
    fn input<T: Into<String>>(msg: T) -> Self {
        FBDError::InputError(msg.into())
    }
}

#[derive(Debug, Clone)]
enum Variable {
    Bool(bool),
    Int(i32),
    Real(f64),
    String(String),
    Date(String),
}


trait FBlock {
    fn apply(&self, inputs: Vec<Variable>) -> FBDResult<Vec<Variable>>;
    fn reset(&mut self);
}

trait Function {
    fn apply(&self, inputs: Vec<Variable>) -> FBDResult<Vec<Variable>>;
}

trait BinaryFunction: Function {
    fn apply_pair(&self, left: Variable, right: Variable) -> FBDResult<Variable>;

    fn apply(&self, inputs: Vec<Variable>) -> FBDResult<Vec<Variable>> {
        if let [lhs, rhs] = &inputs[..] {
            self.apply_pair(lhs.clone(), rhs.clone()).map(|v| vec![v])
        } else {
            Err(FBDError::input(format!(
                "Binary function requires exactly 2 inputs but got {}",
                inputs.len()
            )))
        }
    }
}


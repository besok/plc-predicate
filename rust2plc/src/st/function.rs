use crate::types::TypedValue;
use crate::var::Value;

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    inputs: Vec<Value>,
    outputs: Vec<Value>,
    return_value: TypedValue,
    body: String,

    description: Option<String>,
    namespace: Option<String>,
    version: Option<String>,
}

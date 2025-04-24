use crate::types::TypedValue;


type ValueName = String;

#[derive(Debug, Clone)]
pub enum Value {
    Input(ValueName, TypedValue),     // Named input parameter
    Output(ValueName, TypedValue),    // Named output parameter
    InOut(ValueName, TypedValue),     // Named bidirectional parameter
    Local(ValueName, TypedValue),     // Named internal variable
    Global(ValueName, TypedValue),    // Named system-wide variable
    External(ValueName, TypedValue),  // Named external reference
    Temporary(ValueName, TypedValue), // Named temporary variable
    Return(TypedValue),               // Return value (no name needed as it's the function's return)
    Constant(ValueName, TypedValue),  // Named constant value
}

use crate::st::function::Function;

pub struct PLCRegistry {
    plc_types: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
enum RegistryItem {
    StFn(Function),
}

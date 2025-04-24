use std::fmt::Display;

#[derive(Debug)]
pub enum PLCLang {
    StructuredText,
    LadderDiagram,
    FunctionBlockDiagram,
    SequentialFunctionChart,
}

impl Display for PLCLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PLCLang::StructuredText => write!(f, "StructuredText"),
            PLCLang::LadderDiagram => write!(f, "LadderDiagram"),
            PLCLang::FunctionBlockDiagram => write!(f, "FunctionBlockDiagram"),
            PLCLang::SequentialFunctionChart => write!(f, "SequentialFunctionChart"),
        }
    }
}

impl From<&str> for PLCLang {
    fn from(value: &str) -> Self {
        match value {
            "st" => PLCLang::StructuredText,
            "ld" => PLCLang::LadderDiagram,
            "fbd" => PLCLang::FunctionBlockDiagram,
            "sfc" => PLCLang::SequentialFunctionChart,
            _ => panic!("Unsupported PLC language: {value}, available: st, ld, fbd, sfc"),
        }
    }
}
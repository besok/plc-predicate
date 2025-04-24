use std::time::Duration;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use crate::error::Rust2PlcError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TypedValue {
    // Boolean type
    Bool(bool),

    // Integer types
    SInt(i8),
    Int(i16),
    DInt(i32),
    LInt(i64),
    USInt(u8),
    UInt(u16),
    UDInt(u32),
    ULInt(u64),

    // Floating point types
    Real(f32),
    LReal(f64),

    // Time types
    Time(Duration),
    Date(NaiveDate),
    TimeOfDay(NaiveTime),
    DateTime(NaiveDateTime),

    // Character and string types
    Char(char),
    WChar(char),
    String(String, Option<usize>), // (value, optional max length)
    WString(String, Option<usize>), // (value, optional max length)

    // Bit strings
    Byte(u8),
    Word(u16),
    DWord(u32),
    LWord(u64),

    // User-defined type reference
    UserDefined(String, Option<Box<TypedValue>>), // (type_name, optional contained value)

    // Array type
    Array(Vec<Box<TypedValue>>, Box<TypedValue>), // (values, element_type)
}

impl TypedValue {
    pub fn get_type_name(&self) -> &'static str {
        match self {
            TypedValue::Bool(_) => "Bool",
            TypedValue::SInt(_) => "SInt",
            TypedValue::Int(_) => "Int",
            TypedValue::DInt(_) => "DInt",
            TypedValue::LInt(_) => "LInt",
            TypedValue::USInt(_) => "USInt",
            TypedValue::UInt(_) => "UInt",
            TypedValue::UDInt(_) => "UDInt",
            TypedValue::ULInt(_) => "ULInt",
            TypedValue::Real(_) => "Real",
            TypedValue::LReal(_) => "LReal",
            TypedValue::Time(_) => "Time",
            TypedValue::Date(_) => "Date",
            TypedValue::TimeOfDay(_) => "TimeOfDay",
            TypedValue::DateTime(_) => "DateTime",
            TypedValue::Char(_) => "Char",
            TypedValue::WChar(_) => "WChar",
            TypedValue::String(_, _) => "String",
            TypedValue::WString(_, _) => "WString",
            TypedValue::Byte(_) => "Byte",
            TypedValue::Word(_) => "Word",
            TypedValue::DWord(_) => "DWord",
            TypedValue::LWord(_) => "LWord",
            TypedValue::UserDefined(name, _) => "UserDefined",
            TypedValue::Array(_, _) => "Array",
        }
    }

    pub fn to_rust_type(&self) -> String {
        match self {
            TypedValue::Bool(_) => "bool".to_string(),
            TypedValue::SInt(_) => "i8".to_string(),
            TypedValue::Int(_) => "i16".to_string(),
            TypedValue::DInt(_) => "i32".to_string(),
            TypedValue::LInt(_) => "i64".to_string(),
            TypedValue::USInt(_) => "u8".to_string(),
            TypedValue::UInt(_) => "u16".to_string(),
            TypedValue::UDInt(_) => "u32".to_string(),
            TypedValue::ULInt(_) => "u64".to_string(),
            TypedValue::Real(_) => "f32".to_string(),
            TypedValue::LReal(_) => "f64".to_string(),
            TypedValue::Time(_) => "std::time::Duration".to_string(),
            TypedValue::Date(_) => "chrono::NaiveDate".to_string(),
            TypedValue::TimeOfDay(_) => "chrono::NaiveTime".to_string(),
            TypedValue::DateTime(_) => "chrono::NaiveDateTime".to_string(),
            TypedValue::Char(_) => "char".to_string(),
            TypedValue::WChar(_) => "char".to_string(),
            TypedValue::String(_, _) => "String".to_string(),
            TypedValue::WString(_, _) => "String".to_string(),
            TypedValue::Byte(_) => "u8".to_string(),
            TypedValue::Word(_) => "u16".to_string(),
            TypedValue::DWord(_) => "u32".to_string(),
            TypedValue::LWord(_) => "u64".to_string(),
            TypedValue::UserDefined(name, _) => name.clone(),
            TypedValue::Array(_, elem_type) => format!("[{}; {}]", elem_type.to_rust_type(), self.array_size()),
        }
    }

    pub fn to_plc_type(&self) -> String {
        match self {
            TypedValue::Bool(_) => "BOOL".to_string(),
            TypedValue::SInt(_) => "SINT".to_string(),
            TypedValue::Int(_) => "INT".to_string(),
            TypedValue::DInt(_) => "DINT".to_string(),
            TypedValue::LInt(_) => "LINT".to_string(),
            TypedValue::USInt(_) => "USINT".to_string(),
            TypedValue::UInt(_) => "UINT".to_string(),
            TypedValue::UDInt(_) => "UDINT".to_string(),
            TypedValue::ULInt(_) => "ULINT".to_string(),
            TypedValue::Real(_) => "REAL".to_string(),
            TypedValue::LReal(_) => "LREAL".to_string(),
            TypedValue::Time(_) => "TIME".to_string(),
            TypedValue::Date(_) => "DATE".to_string(),
            TypedValue::TimeOfDay(_) => "TIME_OF_DAY".to_string(),
            TypedValue::DateTime(_) => "DATE_AND_TIME".to_string(),
            TypedValue::Char(_) => "CHAR".to_string(),
            TypedValue::WChar(_) => "WCHAR".to_string(),
            TypedValue::String(_, Some(len)) => format!("STRING[{}]", len),
            TypedValue::String(_, None) => "STRING".to_string(),
            TypedValue::WString(_, Some(len)) => format!("WSTRING[{}]", len),
            TypedValue::WString(_, None) => "WSTRING".to_string(),
            TypedValue::Byte(_) => "BYTE".to_string(),
            TypedValue::Word(_) => "WORD".to_string(),
            TypedValue::DWord(_) => "DWORD".to_string(),
            TypedValue::LWord(_) => "LWORD".to_string(),
            TypedValue::UserDefined(name, _) => name.clone(),
            TypedValue::Array(_, elem_type) => format!("ARRAY[0..{}] OF {}", self.array_size()-1, elem_type.to_plc_type()),
        }
    }

    pub fn array_size(&self) -> usize {
        match self {
            TypedValue::Array(values, _) => values.len(),
            _ => 0,
        }
    }

    pub fn to_plc_literal(&self) -> Result<String, Rust2PlcError> {
        match self {
            TypedValue::Bool(value) => Ok(if *value { "TRUE" } else { "FALSE" }.to_string()),

            // Integer literals
            TypedValue::SInt(value) => Ok(value.to_string()),
            TypedValue::Int(value) => Ok(value.to_string()),
            TypedValue::DInt(value) => Ok(value.to_string()),
            TypedValue::LInt(value) => Ok(value.to_string()),
            TypedValue::USInt(value) => Ok(value.to_string()),
            TypedValue::UInt(value) => Ok(value.to_string()),
            TypedValue::UDInt(value) => Ok(value.to_string()),
            TypedValue::ULInt(value) => Ok(value.to_string()),

            // Real literals
            TypedValue::Real(value) => Ok(format!("{:.6}", value)),
            TypedValue::LReal(value) => Ok(format!("{:.15}", value)),

            // Time literals
            TypedValue::Time(duration) => {
                let secs = duration.as_secs();
                let millis = duration.subsec_millis();

                if millis == 0 {
                    if secs >= 86400 {
                        let days = secs / 86400;
                        let remaining = secs % 86400;
                        if remaining == 0 {
                            Ok(format!("T#{}d", days))
                        } else {
                            let hours = remaining / 3600;
                            let mins = (remaining % 3600) / 60;
                            let secs = remaining % 60;
                            Ok(format!("T#{}d{}h{}m{}s", days, hours, mins, secs))
                        }
                    } else if secs >= 3600 {
                        let hours = secs / 3600;
                        let remaining = secs % 3600;
                        if remaining == 0 {
                            Ok(format!("T#{}h", hours))
                        } else {
                            let mins = remaining / 60;
                            let secs = remaining % 60;
                            Ok(format!("T#{}h{}m{}s", hours, mins, secs))
                        }
                    } else if secs >= 60 {
                        let mins = secs / 60;
                        let secs = secs % 60;
                        if secs == 0 {
                            Ok(format!("T#{}m", mins))
                        } else {
                            Ok(format!("T#{}m{}s", mins, secs))
                        }
                    } else {
                        Ok(format!("T#{}s", secs))
                    }
                } else {
                    Ok(format!("T#{}s{}ms", secs, millis))
                }
            },

            TypedValue::Date(date) => Ok(format!("D#{}", date.format("%Y-%m-%d"))),

            TypedValue::TimeOfDay(time) => Ok(format!("TOD#{}", time.format("%H:%M:%S"))),

            TypedValue::DateTime(dt) => Ok(format!("DT#{}", dt.format("%Y-%m-%d-%H:%M:%S"))),

            // String literals
            TypedValue::Char(c) => Ok(format!("'{}'", c)),
            TypedValue::WChar(c) => Ok(format!("\"{}\"", c)),
            TypedValue::String(value, _) => Ok(format!("'{}'", value)),
            TypedValue::WString(value, _) => Ok(format!("\"{}\"", value)),

            // Bit string literals
            TypedValue::Byte(value) => Ok(format!("16#{}", format!("{:02X}", value))),
            TypedValue::Word(value) => Ok(format!("16#{}", format!("{:04X}", value))),
            TypedValue::DWord(value) => Ok(format!("16#{}", format!("{:08X}", value))),
            TypedValue::LWord(value) => Ok(format!("16#{}", format!("{:016X}", value))),

            // Complex types
            TypedValue::UserDefined(_, Some(value)) => value.to_plc_literal(),
            TypedValue::UserDefined(name, None) => Err(Rust2PlcError::parse(format!("Cannot convert empty user-defined type {} to PLC literal", name))),

            TypedValue::Array(values, _) => {
                let elements: Result<Vec<String>, Rust2PlcError> = values
                    .iter()
                    .map(|v| v.to_plc_literal())
                    .collect();

                match elements {
                    Ok(elements) => Ok(format!("[{}]", elements.join(", "))),
                    Err(e) => Err(e),
                }
            }
        }
    }

    // Create type variants with default values
    pub fn new_bool() -> Self { TypedValue::Bool(false) }
    pub fn new_sint() -> Self { TypedValue::SInt(0) }
    pub fn new_int() -> Self { TypedValue::Int(0) }
    pub fn new_dint() -> Self { TypedValue::DInt(0) }
    pub fn new_lint() -> Self { TypedValue::LInt(0) }
    pub fn new_usint() -> Self { TypedValue::USInt(0) }
    pub fn new_uint() -> Self { TypedValue::UInt(0) }
    pub fn new_udint() -> Self { TypedValue::UDInt(0) }
    pub fn new_ulint() -> Self { TypedValue::ULInt(0) }
    pub fn new_real() -> Self { TypedValue::Real(0.0) }
    pub fn new_lreal() -> Self { TypedValue::LReal(0.0) }
    pub fn new_time() -> Self { TypedValue::Time(Duration::from_secs(0)) }
    pub fn new_date() -> Self {
        TypedValue::Date(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_else(||
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()))
    }
    pub fn new_time_of_day() -> Self {
        TypedValue::TimeOfDay(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }
    pub fn new_date_time() -> Self {
        TypedValue::DateTime(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_else(|| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        ))
    }
    pub fn new_char() -> Self { TypedValue::Char('\0') }
    pub fn new_wchar() -> Self { TypedValue::WChar('\0') }
    pub fn new_string(max_len: Option<usize>) -> Self { TypedValue::String(String::new(), max_len) }
    pub fn new_wstring(max_len: Option<usize>) -> Self { TypedValue::WString(String::new(), max_len) }
    pub fn new_byte() -> Self { TypedValue::Byte(0) }
    pub fn new_word() -> Self { TypedValue::Word(0) }
    pub fn new_dword() -> Self { TypedValue::DWord(0) }
    pub fn new_lword() -> Self { TypedValue::LWord(0) }

    pub fn new_user_defined(name: &str) -> Self {
        TypedValue::UserDefined(name.to_string(), None)
    }

    pub fn new_array(element_type: TypedValue, size: usize) -> Self {
        let mut values = Vec::with_capacity(size);
        let template = match element_type.clone() {
            TypedValue::Bool(_) => Box::new(TypedValue::new_bool()),
            TypedValue::SInt(_) => Box::new(TypedValue::new_sint()),
            TypedValue::Int(_) => Box::new(TypedValue::new_int()),
            TypedValue::DInt(_) => Box::new(TypedValue::new_dint()),
            TypedValue::LInt(_) => Box::new(TypedValue::new_lint()),
            TypedValue::USInt(_) => Box::new(TypedValue::new_usint()),
            TypedValue::UInt(_) => Box::new(TypedValue::new_uint()),
            TypedValue::UDInt(_) => Box::new(TypedValue::new_udint()),
            TypedValue::ULInt(_) => Box::new(TypedValue::new_ulint()),
            TypedValue::Real(_) => Box::new(TypedValue::new_real()),
            TypedValue::LReal(_) => Box::new(TypedValue::new_lreal()),
            TypedValue::Time(_) => Box::new(TypedValue::new_time()),
            TypedValue::Date(_) => Box::new(TypedValue::new_date()),
            TypedValue::TimeOfDay(_) => Box::new(TypedValue::new_time_of_day()),
            TypedValue::DateTime(_) => Box::new(TypedValue::new_date_time()),
            TypedValue::Char(_) => Box::new(TypedValue::new_char()),
            TypedValue::WChar(_) => Box::new(TypedValue::new_wchar()),
            TypedValue::String(_, max_len) => Box::new(TypedValue::new_string(max_len)),
            TypedValue::WString(_, max_len) => Box::new(TypedValue::new_wstring(max_len)),
            TypedValue::Byte(_) => Box::new(TypedValue::new_byte()),
            TypedValue::Word(_) => Box::new(TypedValue::new_word()),
            TypedValue::DWord(_) => Box::new(TypedValue::new_dword()),
            TypedValue::LWord(_) => Box::new(TypedValue::new_lword()),
            TypedValue::UserDefined(name, _) => Box::new(TypedValue::new_user_defined(&name)),
            el => Box::new(el),
        };

        for _ in 0..size {
            values.push(template.clone());
        }

        TypedValue::Array(values, Box::new(element_type))
    }

    pub fn from_rust_type(type_str: &str) -> Option<TypedValue> {
        match type_str {
            "bool" => Some(TypedValue::new_bool()),
            "i8" => Some(TypedValue::new_sint()),
            "i16" => Some(TypedValue::new_int()),
            "i32" => Some(TypedValue::new_dint()),
            "i64" => Some(TypedValue::new_lint()),
            "u8" => Some(TypedValue::new_usint()),
            "u16" => Some(TypedValue::new_uint()),
            "u32" => Some(TypedValue::new_udint()),
            "u64" => Some(TypedValue::new_ulint()),
            "f32" => Some(TypedValue::new_real()),
            "f64" => Some(TypedValue::new_lreal()),
            "std::time::Duration" | "Duration" => Some(TypedValue::new_time()),
            "chrono::NaiveDate" | "NaiveDate" => Some(TypedValue::new_date()),
            "chrono::NaiveTime" | "NaiveTime" => Some(TypedValue::new_time_of_day()),
            "chrono::NaiveDateTime" | "NaiveDateTime" => Some(TypedValue::new_date_time()),
            "char" => Some(TypedValue::new_char()),
            "String" | "&str" => Some(TypedValue::new_string(None)),
            _ => {
                // Check for array types like [T; N]
                if type_str.starts_with('[') && type_str.contains(';') {
                    let parts: Vec<&str> = type_str.trim_matches(|c| c == '[' || c == ']').split(';').collect();
                    if parts.len() == 2 {
                        if let Some(inner_type) = Self::from_rust_type(parts[0].trim()) {
                            if let Ok(size) = parts[1].trim().parse::<usize>() {
                                return Some(TypedValue::new_array(inner_type, size));
                            }
                        }
                    }
                }
                // Fallback to user-defined type
                Some(TypedValue::new_user_defined(type_str))
            }
        }
    }

    pub fn from_plc_type(type_str: &str) -> Option<TypedValue> {
        match type_str.to_uppercase().as_str() {
            "BOOL" => Some(TypedValue::new_bool()),
            "SINT" => Some(TypedValue::new_sint()),
            "INT" => Some(TypedValue::new_int()),
            "DINT" => Some(TypedValue::new_dint()),
            "LINT" => Some(TypedValue::new_lint()),
            "USINT" => Some(TypedValue::new_usint()),
            "UINT" => Some(TypedValue::new_uint()),
            "UDINT" => Some(TypedValue::new_udint()),
            "ULINT" => Some(TypedValue::new_ulint()),
            "REAL" => Some(TypedValue::new_real()),
            "LREAL" => Some(TypedValue::new_lreal()),
            "TIME" => Some(TypedValue::new_time()),
            "DATE" => Some(TypedValue::new_date()),
            "TIME_OF_DAY" | "TOD" => Some(TypedValue::new_time_of_day()),
            "DATE_AND_TIME" | "DT" => Some(TypedValue::new_date_time()),
            "CHAR" => Some(TypedValue::new_char()),
            "WCHAR" => Some(TypedValue::new_wchar()),
            "BYTE" => Some(TypedValue::new_byte()),
            "WORD" => Some(TypedValue::new_word()),
            "DWORD" => Some(TypedValue::new_dword()),
            "LWORD" => Some(TypedValue::new_lword()),
            _ => {
                // Handle STRING with length: STRING[123]
                if type_str.starts_with("STRING[") && type_str.ends_with("]") {
                    let len_str = type_str.trim_start_matches("STRING[").trim_end_matches("]");
                    if let Ok(len) = len_str.parse::<usize>() {
                        return Some(TypedValue::new_string(Some(len)));
                    }
                } else if type_str == "STRING" {
                    return Some(TypedValue::new_string(None));
                }

                // Handle WSTRING with length: WSTRING[123]
                if type_str.starts_with("WSTRING[") && type_str.ends_with("]") {
                    let len_str = type_str.trim_start_matches("WSTRING[").trim_end_matches("]");
                    if let Ok(len) = len_str.parse::<usize>() {
                        return Some(TypedValue::new_wstring(Some(len)));
                    }
                } else if type_str == "WSTRING" {
                    return Some(TypedValue::new_wstring(None));
                }

                // Handle arrays: ARRAY[0..9] OF INT
                if type_str.starts_with("ARRAY[") {
                    let parts: Vec<&str> = type_str.split("OF").collect();
                    if parts.len() == 2 {
                        // Extract array size from ARRAY[0..9]
                        let range_part = parts[0].trim().trim_start_matches("ARRAY[").trim_end_matches("]").trim();
                        let range: Vec<&str> = range_part.split("..").collect();
                        if range.len() == 2 {
                            if let (Ok(start), Ok(end)) = (range[0].parse::<usize>(), range[1].parse::<usize>()) {
                                let size = end - start + 1;
                                if let Some(inner_type) = Self::from_plc_type(parts[1].trim()) {
                                    return Some(TypedValue::new_array(inner_type, size));
                                }
                            }
                        }
                    }
                }

                // Default to user-defined type
                Some(TypedValue::new_user_defined(type_str))
            }
        }
    }
}

impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedValue::Bool(value) => write!(f, "{}", value),
            TypedValue::SInt(value) => write!(f, "{}", value),
            TypedValue::Int(value) => write!(f, "{}", value),
            TypedValue::DInt(value) => write!(f, "{}", value),
            TypedValue::LInt(value) => write!(f, "{}", value),
            TypedValue::USInt(value) => write!(f, "{}", value),
            TypedValue::UInt(value) => write!(f, "{}", value),
            TypedValue::UDInt(value) => write!(f, "{}", value),
            TypedValue::ULInt(value) => write!(f, "{}", value),
            TypedValue::Real(value) => write!(f, "{}", value),
            TypedValue::LReal(value) => write!(f, "{}", value),
            TypedValue::Time(duration) => {
                let secs = duration.as_secs();
                let nanos = duration.subsec_nanos();
                if nanos == 0 {
                    write!(f, "{}s", secs)
                } else {
                    write!(f, "{}.{:09}s", secs, nanos)
                }
            },
            TypedValue::Date(date) => write!(f, "{}", date),
            TypedValue::TimeOfDay(time) => write!(f, "{}", time),
            TypedValue::DateTime(dt) => write!(f, "{}", dt),
            TypedValue::Char(c) => write!(f, "{}", c),
            TypedValue::WChar(c) => write!(f, "{}", c),
            TypedValue::String(value, _) => write!(f, "{}", value),
            TypedValue::WString(value, _) => write!(f, "{}", value),
            TypedValue::Byte(value) => write!(f, "{:#04x}", value),
            TypedValue::Word(value) => write!(f, "{:#06x}", value),
            TypedValue::DWord(value) => write!(f, "{:#010x}", value),
            TypedValue::LWord(value) => write!(f, "{:#018x}", value),
            TypedValue::UserDefined(name, Some(value)) => write!(f, "{}({})", name, value),
            TypedValue::UserDefined(name, None) => write!(f, "{}", name),
            TypedValue::Array(values, _) => {
                write!(f, "[")?;
                for (i, v) in values.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
        }
    }
}

// Implement From traits for common Rust types to TypedValue
impl From<bool> for TypedValue {
    fn from(value: bool) -> Self {
        TypedValue::Bool(value)
    }
}

// Integer type conversions
impl From<i8> for TypedValue {
    fn from(value: i8) -> Self {
        TypedValue::SInt(value)
    }
}

impl From<i16> for TypedValue {
    fn from(value: i16) -> Self {
        TypedValue::Int(value)
    }
}

impl From<i32> for TypedValue {
    fn from(value: i32) -> Self {
        TypedValue::DInt(value)
    }
}

impl From<i64> for TypedValue {
    fn from(value: i64) -> Self {
        TypedValue::LInt(value)
    }
}

impl From<u8> for TypedValue {
    fn from(value: u8) -> Self {
        TypedValue::USInt(value)
    }
}

impl From<u16> for TypedValue {
    fn from(value: u16) -> Self {
        TypedValue::UInt(value)
    }
}

impl From<u32> for TypedValue {
    fn from(value: u32) -> Self {
        TypedValue::UDInt(value)
    }
}

impl From<u64> for TypedValue {
    fn from(value: u64) -> Self {
        TypedValue::ULInt(value)
    }
}

// Floating point type conversions
impl From<f32> for TypedValue {
    fn from(value: f32) -> Self {
        TypedValue::Real(value)
    }
}

impl From<f64> for TypedValue {
    fn from(value: f64) -> Self {
        TypedValue::LReal(value)
    }
}

// Time-related type conversions
impl From<Duration> for TypedValue {
    fn from(value: Duration) -> Self {
        TypedValue::Time(value)
    }
}

impl From<NaiveDate> for TypedValue {
    fn from(value: NaiveDate) -> Self {
        TypedValue::Date(value)
    }
}

impl From<NaiveTime> for TypedValue {
    fn from(value: NaiveTime) -> Self {
        TypedValue::TimeOfDay(value)
    }
}

impl From<NaiveDateTime> for TypedValue {
    fn from(value: NaiveDateTime) -> Self {
        TypedValue::DateTime(value)
    }
}

// Character and string type conversions
impl From<char> for TypedValue {
    fn from(value: char) -> Self {
        TypedValue::Char(value)
    }
}

impl From<String> for TypedValue {
    fn from(value: String) -> Self {
        TypedValue::String(value, None)
    }
}

impl From<&str> for TypedValue {
    fn from(value: &str) -> Self {
        TypedValue::String(value.to_string(), None)
    }
}

// Vec conversions for arrays (basic types)
impl<T: Into<TypedValue> + Clone> From<Vec<T>> for TypedValue {
    fn from(values: Vec<T>) -> Self {
        if values.is_empty() {
            return TypedValue::Array(vec![], Box::new(TypedValue::new_bool()));
        }

        // Create the element type from the first element
        let sample: TypedValue = values[0].clone().into();
        let elem_type = Box::new(sample.clone());

        // Convert all values to TypedValue
        let typed_values = values
            .into_iter()
            .map(|v| Box::new(v.into()))
            .collect();

        TypedValue::Array(typed_values, elem_type)
    }
}

// Array conversions for fixed-size arrays
impl<T: Into<TypedValue> + Clone, const N: usize> From<[T; N]> for TypedValue {
    fn from(array: [T; N]) -> Self {
        if N == 0 {
            return TypedValue::Array(vec![], Box::new(TypedValue::new_bool()));
        }

        // Create the element type from the first element
        let sample: TypedValue = array[0].clone().into();
        let elem_type = Box::new(sample.clone());

        // Convert all values to TypedValue
        let typed_values = array
            .into_iter()
            .map(|v| Box::new(v.into()))
            .collect();

        TypedValue::Array(typed_values, elem_type)
    }
}

// Add specialized string constructors with explicit length
impl TypedValue {
    pub fn string_with_length(value: impl Into<String>, max_length: usize) -> Self {
        TypedValue::String(value.into(), Some(max_length))
    }

    pub fn wstring_with_length(value: impl Into<String>, max_length: usize) -> Self {
        TypedValue::WString(value.into(), Some(max_length))
    }
}
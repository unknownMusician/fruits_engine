use crate::TerminalSerializer;

pub struct StringSerializer;
impl TerminalSerializer<String> for StringSerializer {
    fn serialize(&self, value: String) -> String {
        format!("\"{value}\"")
    }
    
    fn deserialize(&self, data: &str) -> Option<String> {
        if data.len() < 2 {
            return None;
        }

        let mut chars = data.chars();

        if chars.next().unwrap() != '"' {
            return None;
        }

        if chars.last().unwrap() != '"' {
            return None;
        }

        // todo
        let content = &data[1..data.len() - 1];
        Some(String::from(content))
    }
}

pub struct StrSerializer;
impl TerminalSerializer<&'static str> for StrSerializer {
    fn serialize(&self, value: &'static str) -> String {
        format!("\"{value}\"")
    }
    
    fn deserialize(&self, _data: &str) -> Option<&'static str> {
        // todo: unsupported.
        None
    }
}

pub struct BoolSerializer;
impl TerminalSerializer<bool> for BoolSerializer {
    fn serialize(&self, value: bool) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<bool> {
        data.parse().ok()
    }
}

pub struct USizeSerializer;
impl TerminalSerializer<usize> for USizeSerializer {
    fn serialize(&self, value: usize) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<usize> {
        data.parse().ok()
    }
}

pub struct ISizeSerializer;
impl TerminalSerializer<isize> for ISizeSerializer {
    fn serialize(&self, value: isize) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<isize> {
        data.parse().ok()
    }
}

pub struct U8Serializer;
impl TerminalSerializer<u8> for U8Serializer {
    fn serialize(&self, value: u8) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<u8> {
        data.parse().ok()
    }
}

pub struct I8Serializer;
impl TerminalSerializer<i8> for I8Serializer {
    fn serialize(&self, value: i8) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<i8> {
        data.parse().ok()
    }
}

pub struct U16Serializer;
impl TerminalSerializer<u16> for U16Serializer {
    fn serialize(&self, value: u16) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<u16> {
        data.parse().ok()
    }
}

pub struct I16Serializer;
impl TerminalSerializer<i16> for I16Serializer {
    fn serialize(&self, value: i16) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<i16> {
        data.parse().ok()
    }
}

pub struct U32Serializer;
impl TerminalSerializer<u32> for U32Serializer {
    fn serialize(&self, value: u32) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<u32> {
        data.parse().ok()
    }
}

pub struct I32Serializer;
impl TerminalSerializer<i32> for I32Serializer {
    fn serialize(&self, value: i32) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<i32> {
        data.parse().ok()
    }
}

pub struct U64Serializer;
impl TerminalSerializer<u64> for U64Serializer {
    fn serialize(&self, value: u64) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<u64> {
        data.parse().ok()
    }
}

pub struct I64Serializer;
impl TerminalSerializer<i64> for I64Serializer {
    fn serialize(&self, value: i64) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<i64> {
        data.parse().ok()
    }
}

pub struct U128Serializer;
impl TerminalSerializer<u128> for U128Serializer {
    fn serialize(&self, value: u128) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<u128> {
        data.parse().ok()
    }
}

pub struct I128Serializer;
impl TerminalSerializer<i128> for I128Serializer {
    fn serialize(&self, value: i128) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<i128> {
        data.parse().ok()
    }
}

pub struct F32Serializer;
impl TerminalSerializer<f32> for F32Serializer {
    fn serialize(&self, value: f32) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<f32> {
        data.parse().ok()
    }
}

pub struct F64Serializer;
impl TerminalSerializer<f64> for F64Serializer {
    fn serialize(&self, value: f64) -> String {
        value.to_string()
    }
    
    fn deserialize(&self, data: &str) -> Option<f64> {
        data.parse().ok()
    }
}

pub fn register_default_terminals(serializer: &mut crate::GlobalSerializer) {
    serializer.register_terminal(StringSerializer);
    serializer.register_terminal(StrSerializer);
    serializer.register_terminal(BoolSerializer);
    serializer.register_terminal(USizeSerializer);
    serializer.register_terminal(ISizeSerializer);
    serializer.register_terminal(U8Serializer);
    serializer.register_terminal(I8Serializer);
    serializer.register_terminal(U16Serializer);
    serializer.register_terminal(I16Serializer);
    serializer.register_terminal(U32Serializer);
    serializer.register_terminal(I32Serializer);
    serializer.register_terminal(U64Serializer);
    serializer.register_terminal(I64Serializer);
    serializer.register_terminal(U128Serializer);
    serializer.register_terminal(I128Serializer);
    serializer.register_terminal(F32Serializer);
    serializer.register_terminal(F64Serializer);
}

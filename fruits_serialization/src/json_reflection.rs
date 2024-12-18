use std::collections::HashMap;


#[derive(Clone, Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(JsonArray),
    Object(JsonObject),
}

pub enum JsonIndentation {
    Default,
    Indented { indent_level: usize }
}

impl Default for JsonValue {
    fn default() -> Self {
        JsonValue::Null
    }
}

impl JsonValue {
    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        match self {
            JsonValue::Null => String::from("null"),
            JsonValue::Bool(value) => value.to_string(),
            JsonValue::Int(value) => value.to_string(),
            JsonValue::Float(value) => value.to_string(),
            JsonValue::String(value) => format!("\"{}\"", value),
            JsonValue::Array(json) => json.to_json(indentation),
            JsonValue::Object(json) => json.to_json(indentation),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct JsonArray {
    elements: Vec<JsonValue>,
}

impl JsonArray {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn from_elements(elements: &[JsonValue]) -> Self {
        Self {
            elements: Vec::from(elements),
        }
    }

    pub fn elements(&self) -> &Vec<JsonValue> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut Vec<JsonValue> {
        &mut self.elements
    }

    pub fn push_element(&mut self, element: JsonValue) {
        self.elements.push(element);
    }

    pub fn to_json(&self, mut indentation: &mut JsonIndentation) -> String {
        let count = self.elements.len();
        
        if count == 0 {
            return String::from("[]");
        }
        
        let mut json = String::new();

        json.push_str("[");

        adjust_indentation(&mut indentation, 1);
        push_indent(&mut json, indentation);
        
        for (i, element) in self.elements.iter().enumerate() {
            json.push_str(&element.to_json(indentation));
            
            if (i + 1) != count {
                json.push_str(",");
                
                push_indent(&mut json, indentation);
            }
        }
        
        adjust_indentation(&mut indentation, -1);
        push_indent(&mut json, indentation);

        json.push_str("]");

        json
    }
}

impl Into<JsonValue> for JsonArray {
    fn into(self) -> JsonValue {
        JsonValue::Array(self)
    }
}

fn push_indent(json: &mut String, indentation: &JsonIndentation) {
    match indentation {
        JsonIndentation::Default => json.push_str(" "),
        JsonIndentation::Indented { indent_level } => {
            json.push_str("\n");
            for _ in 0..*indent_level {
                json.push_str("  ");
            }
        }
    }
}

fn adjust_indentation(indentation: &mut JsonIndentation, offset: isize) {
    if let JsonIndentation::Indented { indent_level } = indentation {

        *indent_level = match offset < 0 {
            true => {
                let new_offset = -offset as usize;

                if new_offset > *indent_level {
                    panic!("Indentation underflow.");
                }

                *indent_level - new_offset
            },
            false => *indent_level + offset as usize,
        };
    }
}

pub struct JsonField {
    pub name: String,
    pub value: JsonValue,
}

impl JsonField {
    pub fn new(name: String, value: JsonValue) -> Self {
        Self {
            name,
            value,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct JsonObject {
    fields: HashMap<String, JsonValue>,
    field_names: Vec<String>,
}

impl JsonObject {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            field_names: Vec::new(),
        }
    }

    pub fn push_field(&mut self, name: impl Into<String>, value: impl Into<JsonValue>) -> Result<(), JsonField> {
        let field = JsonField::new(name.into(), value.into());

        if self.fields.contains_key(&field.name) {
            return Err(field);
        }

        // todo: here was unwrap
        self.fields.insert(field.name.clone(), field.value);
        self.field_names.push(field.name);

        Ok(())
    }

    pub fn field_names(&self) -> &[String] {
        &self.field_names
    }

    pub fn get_value(&self, name: &str) -> Option<&JsonValue> {
        self.fields.get(name)
    }

    pub fn into_fields(mut self) -> impl Iterator<Item = JsonField> {
        self.field_names.into_iter().map(move |name| {
            let value = self.fields.remove(&name).unwrap();
            JsonField::new(name, value)
        })
    }

    pub fn fields(&self) -> impl Iterator<Item = (&String, &JsonValue)> {
        self.field_names.iter().map(|name| {
            let value = &self.fields[name];
            (name, value)
        })
    }

    // todo: use writer
    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        let count = self.fields.len();

        if count == 0 {
            return String::from("{}");
        }
        
        let mut json = String::new();

        json.push_str("{");

        adjust_indentation(indentation, 1);
        push_indent(&mut json, indentation);

        for (i, (name, element)) in self.fields.iter().enumerate() {
            json.push_str("\"");
            json.push_str(name);
            json.push_str("\": ");
            json.push_str(&element.to_json(indentation));
            
            if (i + 1) != count {
                json.push_str(",");
                push_indent(&mut json, indentation);
            }
        }
        
        adjust_indentation(indentation, -1);
        push_indent(&mut json, indentation);

        json.push_str("}");

        json
    }
}

impl Into<JsonValue> for JsonObject {
    fn into(self) -> JsonValue {
        JsonValue::Object(self)
    }
}

impl Into<JsonValue> for bool {
    fn into(self) -> JsonValue {
        JsonValue::Bool(self)
    }
}

impl Into<JsonValue> for i64 {
    fn into(self) -> JsonValue {
        JsonValue::Int(self)
    }
}

impl Into<JsonValue> for f64 {
    fn into(self) -> JsonValue {
        JsonValue::Float(self)
    }
}

impl Into<JsonValue> for String {
    fn into(self) -> JsonValue {
        JsonValue::String(self)
    }
}

use std::collections::HashMap;

// Serialized (String)
// Reflection (Collections of terminal-values/other-collections)
// Deserialized (<T>)

pub fn convert(json: &str) -> HashMap<String, String> {
    use State as State;

    let mut converter = JsonStringToJsonFieldsConverter {
        fields: HashMap::new(),
        state: State::None,
    };
    
    for (i, c) in json.chars().enumerate() {
        converter.state = converter.handle_char(c, i);
    }

    if converter.state != State::None {
        panic!("Invalid JSON.");
    }

    converter.fields
}

struct JsonStringToJsonFieldsConverter {
    fields: HashMap<String, String>,
    state: State,
}

impl JsonStringToJsonFieldsConverter {
    fn handle_char(&mut self, c: char, i: usize) -> State {
        
        // todo: escape characters
        // todo: arrays

        match &mut self.state {
            State::None => {
                if c.is_whitespace() {
                    return State::None;
                }

                if c == '{' {
                    return State::InsideObject;
                }

                panic!("Invalid JSON. '{}' at {}", c, i);
            },
            State::InsideObject => {
                if c.is_whitespace() {
                    return State::InsideObject;
                }

                if c == '"' {
                    return State::FieldName(String::new());
                }
                
                if c == '}' {
                    return State::None;
                }

                panic!("Invalid JSON.");
            }
            State::FieldName(field_name) => {
                if c == '"' {
                    return State::FieldBeforeColon(std::mem::take(field_name));
                }

                field_name.push(c);

                State::FieldName(std::mem::take(field_name))
            },
            State::FieldBeforeColon(field_name) => {
                if c.is_whitespace() {
                    return State::FieldBeforeColon(std::mem::take(field_name));
                }

                if c == ':' {
                    return State::FieldAfterColon(std::mem::take(field_name));
                }

                panic!("Invalid JSON.");
            }
            State::FieldAfterColon(field_name) => {
                if c.is_whitespace() {
                    return State::FieldAfterColon(std::mem::take(field_name));
                }

                return State::FieldValue {
                    field_name: std::mem::take(field_name),
                    field_value: String::from(c),
                    braces_count: if c == '{' { 1 } else { 0 },
                    brackets_count: if c == '[' { 1 } else { 0 },
                    is_string_literal: c == '"',
                };
            },
            State::FieldValue {
                field_name,
                field_value,
                braces_count,
                brackets_count,
                is_string_literal,
            } => {
                if c == '"' {
                    *is_string_literal = !*is_string_literal;
                }

                if !*is_string_literal {
                    match c {
                        '{' => *braces_count += 1,
                        '}' => *braces_count -= 1,
                        '[' => *brackets_count += 1,
                        ']' => *brackets_count -= 1,
                        _ => (),
                    }
                }

                if !*is_string_literal && *braces_count == 0 && *brackets_count == 0 && (c.is_whitespace() || c == ',') {
                    if c.is_whitespace() {
                        return State::FieldAfterValue(std::mem::take(field_name), std::mem::take(field_value));
                    }
                    if c == ',' {
                        self.fields.insert(std::mem::take(field_name), std::mem::take(field_value));
                        return State::InsideObject;
                    }
                    if c == '}' {
                        return State::None;
                    }
                }

                field_value.push(c);

                return State::FieldValue {
                    field_name: std::mem::take(field_name),
                    field_value: std::mem::take(field_value),
                    braces_count: *braces_count,
                    brackets_count: *brackets_count,
                    is_string_literal: *is_string_literal,
                };
            },
            State::FieldAfterValue(field_name, field_value) => {
                if c.is_whitespace() {
                    return State::FieldAfterValue(std::mem::take(field_name), std::mem::take(field_value));
                }

                if c == ',' {
                    self.fields.insert(std::mem::take(field_name), std::mem::take(field_value));
                    return State::InsideObject;
                }

                if c == '}' {
                    return State::None;
                }

                panic!("Invalid JSON.");
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum State {
    None,
    InsideObject,
    FieldName(String),
    FieldBeforeColon(String),
    FieldAfterColon(String),
    FieldValue {
        field_name: String,
        field_value: String,
        braces_count: usize,
        brackets_count: usize,
        is_string_literal: bool,
    },
    FieldAfterValue(String, String),
    // todo: array
}
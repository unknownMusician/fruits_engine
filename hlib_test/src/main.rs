use std::collections::HashMap;
use std::fmt::Debug;

use fruits_math::{Matrix, Matrix3x3, Matrix4x4, Vec3};

#[derive(Clone, Debug)]
enum JsonElement {
    Null(JsonNull),
    Bool(JsonBool),
    Int(JsonInt),
    Float(JsonFloat),
    String(JsonString),
    Array(JsonArray),
    Object(JsonObject),
}

enum JsonIndentation {
    Default,
    Indented { indent_level: usize }
}

impl Default for JsonElement {
    fn default() -> Self {
        JsonElement::Null(JsonNull)
    }
}

impl JsonElement {
    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        match self {
            JsonElement::Null(json) => json.to_json(indentation),
            JsonElement::Bool(json) => json.to_json(indentation),
            JsonElement::Int(json) => json.to_json(indentation),
            JsonElement::Float(json) => json.to_json(indentation),
            JsonElement::String(json) => json.to_json(indentation),
            JsonElement::Array(json) => json.to_json(indentation),
            JsonElement::Object(json) => json.to_json(indentation),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct JsonNull;
impl JsonNull {
    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        String::from("null")
    }
}
impl Into<JsonElement> for JsonNull {
    fn into(self) -> JsonElement {
        JsonElement::Null(self)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct JsonBool(pub bool);
impl JsonBool {
    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        self.0.to_string()
    }
}
impl Into<JsonElement> for JsonBool {
    fn into(self) -> JsonElement {
        JsonElement::Bool(self)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct JsonInt(pub i32);
impl JsonInt {
    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        self.0.to_string()
    }
}
impl Into<JsonElement> for JsonInt {
    fn into(self) -> JsonElement {
        JsonElement::Int(self)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct JsonFloat(pub f64);
impl JsonFloat {
    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        self.0.to_string()
    }
}
impl Into<JsonElement> for JsonFloat {
    fn into(self) -> JsonElement {
        JsonElement::Float(self)
    }
}

#[derive(Clone, Debug, Default)]
struct JsonString(pub String);
impl JsonString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn to_json(&self, indentation: &mut JsonIndentation) -> String {
        let mut json = String::new();

        json.push_str("\"");
        json.push_str(&self.0);
        json.push_str("\"");

        json
    }
}
impl Into<JsonElement> for JsonString {
    fn into(self) -> JsonElement {
        JsonElement::String(self)
    }
}

#[derive(Clone, Debug, Default)]
struct JsonArray {
    elements: Vec<JsonElement>,
}

impl JsonArray {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn from_elements(elements: &[JsonElement]) -> Self {
        Self {
            elements: Vec::from(elements),
        }
    }

    pub fn push_element(&mut self, element: JsonElement) {
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

impl Into<JsonElement> for JsonArray {
    fn into(self) -> JsonElement {
        JsonElement::Array(self)
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

#[derive(Clone, Debug, Default)]
struct JsonObject {
    fields: HashMap<String, JsonElement>,
}

impl JsonObject {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn insert_field(&mut self, name: impl Into<String>, element: impl Into<JsonElement>) -> Option<JsonElement> {
        self.fields.insert(name.into(), element.into())
    }
    
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

impl Into<JsonElement> for JsonObject {
    fn into(self) -> JsonElement {
        JsonElement::Object(self)
    }
}

fn test_json() {
    let mut json = JsonObject::new();

    json.insert_field("playerName", JsonString::new("Serhii"));
    json.insert_field("playerAge", JsonInt(22));
    json.insert_field("playerHeight", JsonFloat(1.85));
    json.insert_field("isMale", JsonBool(true));
    json.insert_field("relatives", JsonArray::from_elements(&[
        JsonString::new("Ilia").into(),
        JsonString::new("Hlib").into(),
        JsonString::new("Daria").into(),
    ]));

    let result = json.to_json(&mut JsonIndentation::Indented { indent_level: 0 });

    println!("{}", result);
}

fn main() {
    let camera_position = Vec3::new(0.0_f32, 0.0_f32, 0.0_f32);
    let camera_fov = 120_f32;
    let camera_near = 1.0_f32;
    let camera_far = 100_f32;

    let camera_scale_rotation = Matrix3x3::IDENTITY;

    let transform_matrix = fruits_math::into_matrix4x4_with_pos(camera_scale_rotation, camera_position);

    let projection_matrix = fruits_math::perspective_proj_matrix(camera_fov, camera_near, camera_far);

    let matrix_world_to_clip = projection_matrix * transform_matrix.inverse().unwrap();

    //

    let matrix1 = Matrix4x4::<f32>::from_array([
        [1.0, 5.0, 9.0, 4.0],
        [2.0, 6.0, 1.0, 5.0],
        [3.0, 7.0, 2.0, 6.0],
        [4.0, 8.0, 3.0, 7.0],
    ]);

    let matrix2 = Matrix4x4::<f32>::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);

    let result_matrix = matrix1 * matrix2;

    println!("a: {:?}", matrix1.into_array());
    println!("b: {:?}", matrix2.into_array());
    println!("=: {:?}", result_matrix.into_array());

    // debug_transform(Vec3::new(1.0_f32, 1.0_f32, 0.0_f32), matrix_world_to_clip);
    // debug_transform(Vec3::new(1.0_f32, 1.0_f32, 1.0_f32), matrix_world_to_clip);
    // debug_transform(Vec3::new(1.0_f32, 1.0_f32, 2.0_f32), matrix_world_to_clip);
    // debug_transform(Vec3::new(1.0_f32, 1.0_f32, 10.0_f32), matrix_world_to_clip);
    // debug_transform(Vec3::new(1.0_f32, 1.0_f32, 70.0_f32), matrix_world_to_clip);
    // debug_transform(Vec3::new(1.0_f32, 1.0_f32, 100.0_f32), matrix_world_to_clip);
    // debug_transform(Vec3::new(1.0_f32, 1.0_f32, 200.0_f32), matrix_world_to_clip);
}

fn debug_transform(pos: Vec3<f32>, matrix: Matrix4x4<f32>) {
    println!("position: {:?}", pos.into_array());
    println!("transformed: {:?}", matrix.mul_with_projection(pos).into_array());
    println!();
}

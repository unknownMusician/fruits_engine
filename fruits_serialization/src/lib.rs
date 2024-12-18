use std::{any::{Any, TypeId}, collections::HashMap, error::Error, fmt::{Debug, Display}};

pub mod json_reflection;
pub mod terminal;
pub mod json_writing;
pub mod json_deserialization;

static TYPICAL_JSON: &str = r#"
{
    "$type": "dto::Person"
    "name": "Serhii",
    "age": 22,
    "is_developer": true,
    "friends": [
        "Hlib",
        "Illia",
        "Daniel"
    ],
    "profile": {
        "$type": "dto::Profile"
        "email": "serhii@gmail.com",
        "password": "12345678"
    }
}
"#;

// fn write_typical_json(writer: &mut dyn SerializationWriter) {
//     writer.write_string("$type", "dto::Person");
//     writer.write_string("name", "Serhii");
//     writer.write_integer("age", "22");
//     writer.write_bool("is_developer", true);

//     {
//         let friends_writer = writer.write_string_array("friends");

//         friends_writer.write_element("Hlib");
//         friends_writer.write_element("Illia");
//         friends_writer.write_element("Daniel");
//     }

//     {
//         let profile_writer = writer.write_object("profile");

//         profile_writer.write_string("$type", "dto::Profile");
//         profile_writer.write_string("email", "serhii@gmail.com");
//         profile_writer.write_string("password", "12345678");
//     }
// }

pub trait TerminalSerializer<T: 'static> {
    fn serialize(&self, value: T) -> String;
    fn deserialize(&self, data: &str) -> Option<T>;
}

pub struct AbstractTerminalSerializer<T: 'static> {
    serializer: Box<dyn TerminalSerializer<T>>,
}

impl<T: 'static> AbstractTerminalSerializer<T> {
    pub fn new(serializer: impl 'static + TerminalSerializer<T>) -> Self {
        Self {
            serializer: Box::new(serializer),
        }
    }
    pub fn serialize(&self, value: T) -> String {
        self.serializer.serialize(value)
    }
    pub fn deserialize(&self, data: &str) -> Option<T> {
        self.serializer.deserialize(data)
    }
}

pub struct TerminalSerializerRegistry {
    serializers: HashMap<TypeId, Box<dyn Any>>
}

impl TerminalSerializerRegistry {
    pub fn new() -> Self {
        Self {
            serializers: HashMap:: new(),
        }
    }

    pub fn register<T: 'static>(&mut self, serializer: impl 'static + TerminalSerializer<T>) {
        self.serializers.insert(TypeId::of::<T>(), Box::new(AbstractTerminalSerializer::new(serializer)));
    }

    pub fn get<T: 'static>(&self) -> Option<&AbstractTerminalSerializer<T>> {
        let serializer = self.serializers.get(&TypeId::of::<T>())?;
        let serializer = serializer.downcast_ref::<AbstractTerminalSerializer<T>>()?;

        Some(serializer)
    }

    pub fn deserialize<T: 'static>(&self, data: &str) -> Option<T> {
        let Some(serializer) = self.serializers.get(&TypeId::of::<T>()) else {
            return None;
        };
        let Some(serializer) = serializer.downcast_ref::<AbstractTerminalSerializer<T>>() else {
            return None;
        };

        serializer.deserialize(data)
    }
}

pub trait VirtualSerializer {
    fn serialize(&self, ctx: &mut SerializerContext, value: Box<dyn Any>) -> Result<(), SerializationError>;
    fn deserialize(&self, ctx: &mut DeserializerContext) -> Result<Box<dyn Any>, DeserializationError>;
    fn self_as_any(&self) -> &dyn Any;
}

pub trait Serializer<T: 'static> {
    fn serialize(&self, ctx: &mut SerializerContext, value: T) -> Result<(), SerializationError>;
    fn deserialize(&self, ctx: &mut DeserializerContext) -> Result<T, DeserializationError>;
}

pub struct AbstractSerializer<T: 'static> {
    serializer: Box<dyn Serializer<T>>,
}

impl<T: 'static> AbstractSerializer<T> {
    pub fn new(serializer: impl 'static + Serializer<T>) -> Self {
        Self {
            serializer: Box::new(serializer),
        }
    }
    pub fn serialize(&self, ctx: &mut SerializerContext, value: T) -> Result<(), SerializationError> {
        self.serializer.serialize(ctx, value)
    }
    pub fn deserialize(&self, ctx: &mut DeserializerContext) -> Result<T, DeserializationError> {
        self.serializer.deserialize(ctx)
    }
}

impl<T: 'static> VirtualSerializer for AbstractSerializer<T> {
    fn deserialize(&self, ctx: &mut DeserializerContext) -> Result<Box<dyn Any>, DeserializationError> {
        Ok(Box::new(self.deserialize(ctx)?))
    }
    
    fn serialize(&self, ctx: &mut SerializerContext, value: Box<dyn Any>) -> Result<(), SerializationError> {
        let value = *value.downcast::<T>().or(Err(SerializationError { type_name: "<missing>" }))?;
        self.serialize(ctx, value)
    }
    
    fn self_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SerializerRegistry {
    serializers: Vec<Box<dyn VirtualSerializer>>,
    type_id_mapping: HashMap<TypeId, usize>,
    type_name_mapping: HashMap<String, usize>,
}

impl SerializerRegistry {
    pub fn new() -> Self {
        Self {
            serializers: Vec::new(),
            type_id_mapping: HashMap::new(),
            type_name_mapping: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, serializer: impl 'static + Serializer<T>) {
        let serializer = Box::new(AbstractSerializer::new(serializer));

        let index = self.serializers.len();
        
        self.serializers.push(serializer);
        self.type_id_mapping.insert(TypeId::of::<T>(), index);
        self.type_name_mapping.insert(String::from(std::any::type_name::<T>()), index);
    }

    pub fn get<T: 'static>(&self) -> Option<&AbstractSerializer<T>> {
        let index = *self.type_id_mapping.get(&TypeId::of::<T>())?;
        let serializer = self.serializers[index].self_as_any();
        let serializer = serializer.downcast_ref::<AbstractSerializer<T>>().unwrap();

        Some(serializer)
    }

    pub fn get_virtual_by_name(&self, type_name: &str) -> Option<&dyn VirtualSerializer> {
        let index = *self.type_name_mapping.get(type_name)?;
        let serializer = &*self.serializers[index];

        Some(serializer)
    }

    pub fn get_virtual_by_id(&self, type_id: &TypeId) -> Option<&dyn VirtualSerializer> {
        let index = *self.type_id_mapping.get(type_id)?;
        let serializer = &*self.serializers[index];

        Some(serializer)
    }
}

pub struct GlobalSerializer {
    terminal_serializers: TerminalSerializerRegistry,
    serializers: SerializerRegistry,
}

pub struct SerializationError {
    type_name: &'static str,
}
impl Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SerializationError {{ {} }}", self.type_name)
    }
}
impl Debug for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <SerializationError as Display>::fmt(&self, f)
    }
}
impl Error for SerializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub enum DeserializationError {
    NoSerializerRegistered { type_name: String },
    InvalidInput,
}
impl Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSerializerRegistered { type_name } => write!(f, "NoSerializerRegistered {{ {} }}", type_name),
            Self::InvalidInput => write!(f, "InvalidInput"),
        }
    }
}
impl Debug for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <DeserializationError as Display>::fmt(&self, f)
    }
}
impl Error for DeserializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl GlobalSerializer {
    pub fn new() -> Self {
        Self {
            serializers: SerializerRegistry::new(),
            terminal_serializers: TerminalSerializerRegistry::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, serializer: impl 'static + Serializer<T>) {
        self.serializers.register(serializer)
    }

    pub fn register_terminal<T: 'static>(&mut self, serializer: impl 'static + TerminalSerializer<T>) {
        self.terminal_serializers.register(serializer)
    }

    pub fn serialize<T: 'static>(&self, value: T) -> Result<String, SerializationError> {
        let mut ctx = SerializerContext {
            buffer: String::new(),
            registry: &self.serializers,
            terminal_registry: &self.terminal_serializers,
        };

        ctx.serialize_value(value)?;
        
        Ok(ctx.take_buffer())
    }

    pub fn serialize_virtual(&self, value: Box<dyn Any>) -> Result<String, SerializationError> {
        let mut ctx = SerializerContext {
            buffer: String::new(),
            registry: &self.serializers,
            terminal_registry: &self.terminal_serializers,
        };

        let Some(serializer) = self.serializers.get_virtual_by_id(&Any::type_id(&*value)) else {
            return Err(SerializationError { type_name: "<missing>" });
        };

        // todo: move to ctx
        serializer.serialize(&mut ctx, value)?;

        Ok(ctx.take_buffer())
    }

    pub fn deserialize<T: 'static>(&self, data: &str) -> Result<T, DeserializationError> {
        let mut ctx = DeserializerContext::from_json(data, &self.serializers, &self.terminal_serializers);

        ctx.deserialize_value()
    }

    pub fn deserialize_virtual(&self, data: &str) -> Result<Box<dyn Any>, DeserializationError> {
        let mut ctx = DeserializerContext::from_json(data, &self.serializers, &self.terminal_serializers);

        let type_name = &ctx.fields["$type"];

        // todo
        let type_name = &type_name[1..type_name.len() - 1];

        // todo: move to ctx
        let Some(serializer) = self.serializers.get_virtual_by_name(type_name) else {
            return Err(DeserializationError::NoSerializerRegistered { type_name: String::from(type_name) })
        };

        serializer.deserialize(&mut ctx)
    }
}

pub struct SerializerContext<'r> {
    buffer: String,
    terminal_registry: &'r TerminalSerializerRegistry,
    registry: &'r SerializerRegistry,
}
impl<'r> SerializerContext<'r> {
    pub fn serialize_field<T: 'static>(&mut self, name: String, value: T) -> Result<(), SerializationError> {
        self.buffer.push_str("\"");
        self.buffer.push_str(&name);
        self.buffer.push_str("\": ");

        self.serialize_value(value)?;

        self.buffer.push_str(", ");

        Ok(())
    }

    pub fn serialize_value<T: 'static>(&mut self, value: T) -> Result<(), SerializationError> {
        match self.terminal_registry.get::<T>() {
            Some(serializer) => {
                self.buffer.push_str(&serializer.serialize(value));
                return Ok(());
            },
            None => {
                match self.registry.get() {
                    None => return Err(SerializationError { type_name: std::any::type_name::<T>() }),
                    Some(serializer) => {
                        self.buffer.push_str("{ ");

                        serializer.serialize(self, value)?;

                        self.buffer.push_str(" }");

                        return Ok(());
                    }
                }
            }
        }
    }

    pub fn take_buffer(self) -> String {
        self.buffer
    }
}

pub struct DeserializerContext<'r> {
    fields: HashMap<String, String>,
    registry: &'r SerializerRegistry,
    terminal_registry: &'r TerminalSerializerRegistry,
}
impl<'r> DeserializerContext<'r> {
    pub fn from_json(json: &str, registry: &'r SerializerRegistry, terminal_registry: &'r TerminalSerializerRegistry) -> Self {
        Self {
            fields: json_deserialization::convert(json),
            registry,
            terminal_registry,
        }
    }

    pub fn deserialize_value<T: 'static>(&mut self) -> Result<T, DeserializationError> {
        let serializer = self.registry.get().ok_or_else(|| {
            DeserializationError::NoSerializerRegistered { type_name: String::from(std::any::type_name::<T>()) }
        })?;
        serializer.deserialize(self)
    }

    pub fn deserialize_field<T: 'static>(&mut self, name: String) -> Result<T, DeserializationError> {
        let field_value = self.fields.remove(&name).ok_or(DeserializationError::InvalidInput)?;

        match self.terminal_registry.get::<T>() {
            Some(terminal_serializer) => {
                terminal_serializer.deserialize(&field_value).ok_or(DeserializationError::InvalidInput)
            },
            None => {
                let mut ctx = DeserializerContext::from_json(&field_value, self.registry, self.terminal_registry);

                ctx.deserialize_value()
            }
        }
    }
}
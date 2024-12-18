use fruits_serialization::{json_writing::*, DeserializationError, DeserializerContext, GlobalSerializer, SerializationError, Serializer, SerializerContext};

fn test_serialization() {
    let mut serializer = GlobalSerializer::new();

    fruits_serialization::terminal::register_default_terminals(&mut serializer);
    
    serializer.register(SampleStructSerializer);

    //

    const SAVED_JSON: &'static str = r#"
    {
        "$type": "hlib_test::SampleStruct",
        "age": 68,
        "name": "Dmytro",
    }
    "#;

    let sample_struct = serializer.deserialize::<SampleStruct>(&SAVED_JSON).unwrap();

    let serialized = serializer.serialize(sample_struct);

    println!("{}", serialized.unwrap());

    //

    let sample_struct = serializer.deserialize_virtual(&SAVED_JSON).unwrap();

    let serialized = serializer.serialize_virtual(sample_struct);

    println!("{:?}", serialized);
}

pub fn test_json() {
    let mut buffer = String::new();

    {
        let mut writer = JsonObjectWriter::new(&mut buffer, Some(1));

        writer.write_field("null_field").write_null();
        {
            let mut writer = writer.write_field("array_field").write_array();

            writer.write_element();
            writer.write_element().write_bool(false);
            writer.write_element().write_int(123);
        }
        writer.write_field("int_field").write_int(5);
        writer.write_field("float_field").write_float(8.956);
        writer.write_field("bool_field").write_bool(false);
        writer.write_field("string_field").write_string("hello, world");
        {
            let mut writer = writer.write_field("object_field").write_object();
            
            writer.write_field("name").write_string("Serhii");
            writer.write_field("age").write_int(5);
        }
        {
            writer.write_field("empty_object_field").write_object();
            writer.write_field("empty_array_field").write_array();
            
        }
    }

    println!("{}", buffer);
}


pub struct SampleStruct {
    pub name: String,
    pub age: u8,
}

pub struct SampleStructSerializer;
impl Serializer<SampleStruct> for SampleStructSerializer {
    fn serialize(&self, ctx: &mut SerializerContext, value: SampleStruct) -> Result<(), SerializationError> {
        ctx.serialize_field(String::from("age"), value.age)?;
        ctx.serialize_field(String::from("name"), value.name)?;

        Ok(())
    }

    fn deserialize(&self, ctx: &mut DeserializerContext) -> Result<SampleStruct, DeserializationError> {
        Ok(SampleStruct {
            age: ctx.deserialize_field(String::from("age"))?,
            name: ctx.deserialize_field(String::from("name"))?,
        })
    }
}

fn push_indents(buffer: &mut String, count: usize) {
    for _ in 0..count {
        buffer.push_str("    ");
    }
}

pub trait WriteElementApproveReceiver {
    fn approve(&mut self, buffer: &mut String);
}

//

pub struct JsonObjectWriter<'b> {
    buffer: &'b mut String,
    prefix_writer: JsonObjectFieldPrefixWriter,
    indent: Option<usize>,
}

impl<'b> JsonObjectWriter<'b> {
    pub fn new(buffer: &'b mut String, indent: Option<usize>) -> Self {
        buffer.push_str("{");

        Self {
            buffer,
            prefix_writer: JsonObjectFieldPrefixWriter::new(),
            indent,
        }
    }

    pub fn write_field(&mut self, name: &str) -> JsonValueWriter<JsonObjectWriterApprover> {
        let incremented_indent = self.indent.map(|i| i + 1);

        let approve_receiver = self.prefix_writer.create_approver(name.to_string(), incremented_indent);

        JsonValueWriter::new(self.buffer, approve_receiver, incremented_indent)
    }
}

impl<'b> Drop for JsonObjectWriter<'b> {
    fn drop(&mut self) {
        if self.prefix_writer.did_write_one_field {
            if let Some(some_indent) = self.indent {
                self.buffer.push_str("\n");
                push_indents(self.buffer, some_indent);
            } else {
                self.buffer.push_str(" ");
            }
        }

        self.buffer.push_str("}");
    }
}

struct JsonObjectFieldPrefixWriter {
    did_write_one_field: bool,
}

impl JsonObjectFieldPrefixWriter {
    pub fn new() -> Self {
        Self {
            did_write_one_field: false,
        }
    }

    pub fn create_approver(&mut self, name: String, indent: Option<usize>) -> JsonObjectWriterApprover {
        JsonObjectWriterApprover::new(self, name, indent)
    }
}

pub struct JsonObjectWriterApprover<'w> {
    writer: &'w mut JsonObjectFieldPrefixWriter,
    name: String,
    indent: Option<usize>,
}

impl<'w> JsonObjectWriterApprover<'w> {
    fn new(writer: &'w mut JsonObjectFieldPrefixWriter, name: String, indent: Option<usize>) -> Self {
        Self {
            writer,
            name,
            indent,
        }
    }
}

impl<'w> WriteElementApproveReceiver for JsonObjectWriterApprover<'w> {
    fn approve(&mut self, buffer: &mut String) {
        if self.writer.did_write_one_field {
            buffer.push_str(",");
        }
        
        if let Some(some_indent) = self.indent {
            buffer.push_str("\n");
            push_indents(buffer, some_indent);
        } else {
            buffer.push_str(" ");
        }

        buffer.push_str("\"");
        buffer.push_str(&self.name);
        buffer.push_str("\": ");

        self.writer.did_write_one_field = true;
    }
}

//

pub struct JsonArrayWriter<'b> {
    buffer: &'b mut String,
    prefix_writer: JsonArrayFieldPrefixWriter,
    indent: Option<usize>,
}

impl<'b> JsonArrayWriter<'b> {
    pub fn new(buffer: &'b mut String, indent: Option<usize>,) -> Self {
        buffer.push_str("[");

        Self {
            buffer,
            prefix_writer: JsonArrayFieldPrefixWriter::new(),
            indent,
        }
    }

    pub fn write_element(&mut self) -> JsonValueWriter<JsonArrayWriterApprover> {
        let incremented_indent = self.indent.map(|i| i + 1);

        let approver = self.prefix_writer.create_approver(incremented_indent);

        JsonValueWriter::new(self.buffer, approver, incremented_indent)
    }
}

impl<'b> Drop for JsonArrayWriter<'b> {
    fn drop(&mut self) {
        if self.prefix_writer.did_write_one_element {
            if let Some(some_indent) = self.indent {
                self.buffer.push_str("\n");
                push_indents(self.buffer, some_indent);
            } else {
                self.buffer.push_str(" ");
            }
        }
        
        self.buffer.push_str("]");
    }
}

struct JsonArrayFieldPrefixWriter {
    did_write_one_element: bool,
}

impl JsonArrayFieldPrefixWriter {
    pub fn new() -> Self {
        Self {
            did_write_one_element: false,
        }
    }

    pub fn create_approver(&mut self, indent: Option<usize>) -> JsonArrayWriterApprover {
        JsonArrayWriterApprover::new(self, indent)
    }
}

pub struct JsonArrayWriterApprover<'w> {
    writer: &'w mut JsonArrayFieldPrefixWriter,
    indent: Option<usize>,
}

impl<'w> JsonArrayWriterApprover<'w> {
    fn new(writer: &'w mut JsonArrayFieldPrefixWriter, indent: Option<usize>) -> Self {
        Self {
            writer,
            indent,
        }
    }
}

impl<'w> WriteElementApproveReceiver for JsonArrayWriterApprover<'w> {
    fn approve(&mut self, buffer: &mut String) {
        if self.writer.did_write_one_element {
            buffer.push_str(",");
        }

        if let Some(some_indent) = self.indent {
            buffer.push_str("\n");
            push_indents(buffer, some_indent);
        } else {
            buffer.push_str(" ");
        }

        self.writer.did_write_one_element = true;
    }
}

//

pub struct JsonValueWriter<'b, W: WriteElementApproveReceiver> {
    buffer: &'b mut String,
    approve_receiver: W,
    indent: Option<usize>,
}

impl<'b, W: WriteElementApproveReceiver> JsonValueWriter<'b, W> {
    pub fn new(buffer: &'b mut String, approve_receiver: W, indent: Option<usize>) -> Self {
        Self {
            buffer,
            approve_receiver,
            indent,
        }
    }

    pub fn write_object(mut self) -> JsonObjectWriter<'b> {
        self.approve();

        JsonObjectWriter::new(self.buffer, self.indent)
    }
    pub fn write_array(mut self) -> JsonArrayWriter<'b> {
        self.approve();
        
        JsonArrayWriter::new(self.buffer, self.indent)
    }
    pub fn write_string(mut self, value: &str) {
        self.approve();
        
        self.buffer.push_str("\"");
        self.buffer.push_str(value);
        self.buffer.push_str("\"");
    }
    pub fn write_float(mut self, value: f64) {
        self.approve();
        
        self.buffer.push_str(&value.to_string());
    }
    pub fn write_int(mut self, value: i64) {
        self.approve();
        
        self.buffer.push_str(&value.to_string());
    }
    pub fn write_bool(mut self, value: bool) {
        self.approve();

        let text = match value {
            true => "true",
            false => "false",
        };
        
        self.buffer.push_str(text);
    }
    pub fn write_null(mut self) {
        self.approve();
        
        self.buffer.push_str("null");
    }

    fn approve(&mut self) {
        self.approve_receiver.approve(self.buffer);
    }
}




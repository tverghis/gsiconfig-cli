/// Represents an object that can serialize other arbitratry structs into the GSI-specific format.
pub struct Serializer {

    /// Stores the current state of the object that is being serialized.
    pub output: String,

    /// Number of indents (in our case, tab characters) that should be used to indent the **fields** of a struct.
    /// The methods for the `Serializer` struct will automatically take care of correctly indenting other tokens,
    /// like surrounding curly braces.
    pub indent: usize,
}

/// Represents any type that can be serialized by `Serializer`.
pub trait Serializable {
    fn serialize(&self) -> String;
}

/// Very simple wrappers around native Rust types.
/// Used by `serialize_field` to correctly determine the output format.
pub enum FieldType {

    /// Represents a serializable `String` field
    FString(String),

    /// Represents a serializable `bool` field
    FBool(bool),

    /// Represents a serializable `f32` field
    FFloat(f32),
}

impl Serializer {
    /// Begins serialization for a struct.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the object being serialized. This serves as the title of a collection
    /// of fields when written to file. Examples: `"data"`, `"auth"`.
    pub fn serialize_struct(&mut self, name: &'static str) {
        let indenter = "\t".repeat(self.indent - 1);
        self.output += &format!("{}\"{}\"\n", indenter, name);
        self.output += &format!("{}{{\n", indenter);
    }

    /// Serializes a struct's field to the GSI config format. 
    /// This works on a limited subset of types (see `FieldType`).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field being serialized.
    /// * `value` - The value of the field being serialized.
    pub fn serialize_field(&mut self, name: &'static str, value: FieldType) {
        let indenter = "\t".repeat(self.indent);
        match value {
            FieldType::FString(v) => {
                self.output += &format!("{}\"{}\"\t\"{}\"\n", indenter, name, v)
            }
            FieldType::FFloat(v) => {
                self.output += &format!("{}\"{}\"\t\"{:.1}\"\n", indenter, name, v)
            }
            FieldType::FBool(v) => {
                if v {
                    self.output += &format!("{}\"{}\"\t\"1\"\n", indenter, name);
                } else {
                    self.output += &format!("{}\"{}\"\t\"0\"\n", indenter, name);
                }
            }
        };
    }

    /// Directly adds an already-serialized field (example a nested struct) into the current output state.
    /// Useful if `serialize_field` cannot directly handle your type.
    ///
    /// # Arguments
    ///
    /// * `value` - The already-serialized field to be inserted.
    pub fn add_serialized_field(&mut self, value: &str) {
        self.output += value;
    }

    /// Terminates the serialized output.
    ///
    /// # Arguments
    ///
    /// * `newline` - If true, appends a newline character after termination.
    pub fn end(&mut self, newline: bool) -> String {
        let indenter = "\t".repeat(self.indent - 1);
        self.output += &format!("{}}}", indenter);
        if newline {
            self.output += "\n";
        }
        self.output.clone()
    }
}

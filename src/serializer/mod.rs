pub struct Serializer {
    pub output: String,
    pub indent: usize,
}

pub trait Serializable {
    fn serialize(&self) -> String;
}

pub enum FieldType {
    FString(String),
    FBool(bool),
    FFloat(f32),
}

impl Serializer {
    pub fn serialize_struct(&mut self, name: &'static str) {
        let indenter = "\t".repeat(self.indent - 1);
        self.output += &format!("{}\"{}\"\n", indenter, name);
        self.output += &format!("{}{{\n", indenter);
    }

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

    pub fn add_serialized_field(&mut self, value: &str) {
        self.output += value;
    }

    pub fn end(&mut self, newline: bool) -> String {
        let indenter = "\t".repeat(self.indent - 1);
        self.output += &format!("{}}}", indenter);
        if newline {
            self.output += "\n";
        }
        self.output.clone()
    }
}

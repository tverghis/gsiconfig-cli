use serializer::{Serializer, Serializable, FieldType};

pub struct AuthData {
    pub token: String
}

impl Serializable for AuthData {
    fn serialize(&self) -> String {
        let mut serializer = Serializer { output: String::new(), indent: 2 };
        serializer.serialize_struct("auth");
        serializer.serialize_field("token", FieldType::FString(self.token.clone()));
        serializer.end(true)
    }
}
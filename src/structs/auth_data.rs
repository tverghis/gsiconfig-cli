use serializer::{Serializer, Serializable, FieldType};

/// Represents an authentication token that can be sent to the endpoint to validate requests.
///
/// In LAN settings, the token is probably not needed.
/// If sending requests in non-LAN settings, consider using SSL to protect the integrity of the token.
pub struct AuthData {

    /// A token that can be used to authenticate requests to the endpoint.
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
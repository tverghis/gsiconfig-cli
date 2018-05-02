use serializer::{Serializer, Serializable, FieldType};

/// Represents authentication tokens that can be sent to the endpoint to validate requests.
///
/// In LAN settings, these tokens are probably not needed.
/// If sending requests in non-LAN settings, consider using SSL to protect the integrity of these tokens.
pub struct AuthData {

    /// A token that can be used to authenticate requests to the endpoint.
    ///
    /// **NOTE:** This should be a `Vec<String>` to support multiple tokens.
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
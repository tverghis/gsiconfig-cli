use url::Url;

use serializer::{Serializer, Serializable, FieldType};
use structs::request_data::RequestData;
use structs::auth_data::AuthData;

pub struct Config {
    pub uri: Url,
    pub timeout: f32,
    pub buffer: f32,
    pub throttle: f32,
    pub heartbeat: f32,
    pub request_data: RequestData,
    pub auth_data: AuthData,
}

impl Serializable for Config {
    fn serialize(&self) -> String {
        let mut serializer = Serializer { output: String::new(), indent: 1 };
        serializer.serialize_struct("dota2-gsi Configuration");
        serializer.serialize_field("uri", FieldType::FString(self.uri.as_str().to_string()));
        serializer.serialize_field("timeout", FieldType::FFloat(self.timeout));
        serializer.serialize_field("buffer", FieldType::FFloat(self.buffer));
        serializer.serialize_field("throttle", FieldType::FFloat(self.throttle));
        serializer.serialize_field("heartbeat", FieldType::FFloat(self.heartbeat));
        serializer.add_serialized_field(&self.request_data.serialize());
        serializer.add_serialized_field(&self.auth_data.serialize());
        serializer.end(false)
    }
}
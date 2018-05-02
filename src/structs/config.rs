use url::Url;

use serializer::{Serializer, Serializable, FieldType};
use structs::request_data::RequestData;
use structs::auth_data::AuthData;

/// Represents the overall structure of the configuration file.
/// Contains some details about the listening endpoint, as well as what information to send.
///
/// A lot of detailed information about the fields can be found [here](https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration#Endpoint_Section_Settings).
pub struct Config {

    /// The address of the endpoint that is listening for GSI requests.
    pub uri: Url,

    /// Number of seconds before the game should consider requests as having timed out.
    pub timeout: f32,

    /// Number of seconds to collect events before sending them to the server.
    /// Can be low/0 for LAN endpoints.
    pub buffer: f32,

    /// Minimum number of seconds to wait between sending requests to the server.
    pub throttle: f32,

    /// If no game event updates happen after a successful response,
    /// the game will wait `heartbeat` seconds before pinging the server again.
    pub heartbeat: f32,

    /// The game data that the game should send to the endpoint.
    pub request_data: RequestData,

    /// Tokens that can be used to authenticate the payload in non-LAN settings.
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
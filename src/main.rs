extern crate gsicfg;
extern crate url;

use gsicfg::serializer::{Serializer, Serializable, FieldType};
use gsicfg::utils;

use url::Url;

struct Config {
    uri: Url,
    timeout: f32,
    buffer: f32,
    throttle: f32,
    heartbeat: f32,
    request_data: RequestData,
    auth_data: AuthData,
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

struct RequestData {
    buildings: bool,
    provider: bool,
    map: bool,
    player: bool,
    hero: bool,
    abilities: bool,
    items: bool,
    draft: bool,
    wearables: bool,
}

impl Serializable for RequestData {
    fn serialize(&self) -> String {
        let mut serializer = Serializer { output: String::new(), indent: 2 };
        serializer.serialize_struct("data");
        if self.buildings { serializer.serialize_field("buildings", FieldType::FBool(self.buildings)); }
        if self.provider {serializer.serialize_field("provider", FieldType::FBool(self.provider)); }
        if self.map { serializer.serialize_field("map", FieldType::FBool(self.map)); }
        if self.player { serializer.serialize_field("player", FieldType::FBool(self.player)); }
        if self.hero { serializer.serialize_field("hero", FieldType::FBool(self.hero)); }
        if self.abilities { serializer.serialize_field("abilities", FieldType::FBool(self.abilities)); }
        if self.items { serializer.serialize_field("items", FieldType::FBool(self.items)); }
        if self.draft { serializer.serialize_field("draft", FieldType::FBool(self.draft)); }
        if self.wearables { serializer.serialize_field("wearables", FieldType::FBool(self.wearables)); }
        serializer.end(true)
    }
}

struct AuthData {
    token: String
}

impl Serializable for AuthData {
    fn serialize(&self) -> String {
        let mut serializer = Serializer { output: String::new(), indent: 2 };
        serializer.serialize_struct("auth");
        serializer.serialize_field("token", FieldType::FString(self.token.clone()));
        serializer.end(true)
    }
}

fn main() {
    let url = match Url::parse("http://localhost:8080/") {
        Err(why) => panic!("Unable to parse the URL: {}.", why),
        Ok(url) => url
    };

    let cfg = Config {
        uri: url,
        timeout: 5.0,
        buffer: 0.1,
        throttle: 0.1,
        heartbeat: 30.0,
        request_data: RequestData {
            buildings: true,
            provider: true,
            map: true,
            abilities: true,
            hero: true,
            draft: true,
            items: true,
            player: true,
            wearables: true
        },
        auth_data: AuthData {
            token: String::from("hello123")
        }
    };

    match utils::write_cfg("./out/", None, &cfg.serialize()) {
        Err(why) => panic!("Error opening or writing to the file: {}.", why),
        Ok(path) => println!("Successfully wrote to {}.", path),
    }
}
extern crate gsicfg;
extern crate url;

use url::Url;

use gsicfg::utils;
use gsicfg::structs::config::Config;
use gsicfg::structs::auth_data::AuthData;
use gsicfg::structs::request_data::RequestData;
use gsicfg::serializer::Serializable;

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
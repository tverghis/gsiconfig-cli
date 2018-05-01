extern crate url;

use url::Url;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Config {
    uri: Url,
    timeout: f32,
    buffer: f32,
    throttle: f32,
    heartbeat: f32,
    request_data: RequestData
}

impl Config {
    fn serialize(&self) -> String {
        format!(
            "\
                \"dota2-gsi Configuration\"\n\
                {{\n\
                    \t{uri}\n\
                    \t{timeout}\n\
                    \t{buffer}\n\
                    \t{throttle}\n\
                    \t{heartbeat}\n\
                    {request_data}\n\
                }}
            ",
            uri=self.uri.as_str().cfg_out("uri"),
            timeout=self.timeout.cfg_out("timeout"),
            buffer=self.buffer.cfg_out("buffer"),
            throttle=self.throttle.cfg_out("throttle"),
            heartbeat=self.heartbeat.cfg_out("heartbeat"),
            request_data=self.request_data.serialize()
                                .lines()
                                .map(|line| format!("\t{}\n", line))
                                .collect::<String>().trim_right(),
        )
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
    wearables: bool
}

impl RequestData {
    fn serialize(&self) -> String {
        let mut output = String::from("\"data\"\n");
        output.push_str("{\n");

        if self.buildings {
            output.push_str(&format!("\t{}\n", self.buildings.cfg_out("buildings")));
        }

        if self.provider {
            output.push_str(&format!("\t{}\n", self.provider.cfg_out("provider")));
        }

        if self.map {
            output.push_str(&format!("\t{}\n", self.map.cfg_out("map")));
        }
        
        if self.player {
            output.push_str(&format!("\t{}\n", self.player.cfg_out("player")));
        }

        if self.hero {
            output.push_str(&format!("\t{}\n", self.hero.cfg_out("hero")));
        }

        if self.abilities {
            output.push_str(&format!("\t{}\n", self.abilities.cfg_out("abilities")));
        }

        if self.items {
            output.push_str(&format!("\t{}\n", self.items.cfg_out("items")));
        }
        
        if self.draft {
            output.push_str(&format!("\t{}\n", self.draft.cfg_out("draft")));
        }

        if self.wearables {
            output.push_str(&format!("\t{}\n", self.wearables.cfg_out("wearables")));
        }

        output.push_str("}");
        output
    }
}

trait WriteableField {
    fn cfg_out(&self, field_name: &str) -> String;
}

impl<'a> WriteableField for &'a str {
    fn cfg_out(&self, field_name: &str) -> String {
        format!("\"{}\"\t\"{}\"", field_name, self)
    }
}

impl WriteableField for f32 {
    fn cfg_out(&self, field_name: &str) -> String {
        format!("\"{}\"\t\"{:.1}\"", field_name, self)
    }
}

impl WriteableField for bool {
    fn cfg_out(&self, field_name: &str) -> String {
        format!("\"{}\"\t\"1\"", field_name)
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
        }
    };

    match write_cfg("./out/", None, &cfg.serialize()) {
        Err(why) => panic!("Error opening or writing to the file: {}.", why),
        Ok(path) => println!("Successfully wrote to {}.", path),
    }
}

/// Writes the contents of the config string to an appropriately-named file.
///
/// Returns a `std::io::Result<String>` containing the location of the file if the write succeeded.
/// Bubbles up errors from trying to create (and write to) the necessary directories and files.
///
/// # Arguments
///
/// * `location` - The path to the file (minus the name of the file - this is handled for you)
///
/// * `service_name` - Optionally specify the name of your service. If `None`, it will default to `dota2-gsi`.
/// 
/// * `contents` - The contents (presumably the config string) to write to the file.
pub fn write_cfg(location: &str, service_name: Option<&str>, contents: &str) -> std::io::Result<String> {
    fs::create_dir_all(location)?;

    let service_name = match service_name {
        Some(n) => n,
        None => "dota2-gsi"
    };

    let filename = format!("{}gamestate_integration_{}.cfg", location, service_name);
    let path = Path::new(&filename);

    let mut file = File::create(&path)?;

    file.write_all(contents.as_bytes())?;

    Ok(path.display().to_string())
}

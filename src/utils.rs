use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
pub fn write_cfg(
    location: &str,
    service_name: Option<&str>,
    contents: &str,
) -> io::Result<String> {
    fs::create_dir_all(location)?;

    let service_name = match service_name {
        Some(n) => n,
        None => "dota2-gsi",
    };

    let filename = format!("{}gamestate_integration_{}.cfg", location, service_name);
    let path = Path::new(&filename);

    let mut file = File::create(&path)?;

    file.write_all(contents.as_bytes())?;

    Ok(path.display().to_string())
}

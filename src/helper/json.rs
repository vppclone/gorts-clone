use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_reader, from_value, to_value, Map, Value};
use std::fs::File;
use std::io::Write;

pub fn patch_struct<T: DeserializeOwned + Serialize>(
    target: &mut T,
    patch: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut target_map: Map<String, Value> = from_value(to_value(&target)?)?;
    let patch_map: Map<String, Value> = from_value(to_value(&patch)?)?;

    for (key, value) in patch_map {
        if value != Value::Null {
            target_map.insert(key, value);
        }
    }

    *target = from_value(Value::Object(target_map))?;
    Ok(())
}

pub fn read_json<T: DeserializeOwned + Serialize>(url: &str) -> T {
    let file = File::open(url).expect("file should open read only");
    let object: T = from_reader(file).expect("file should be proper JSON");
    object
}

pub fn write_json<T: DeserializeOwned + Serialize>(object: &T, url: &str) -> () {
    let mut file = File::create(url).expect("file should create properly");

    file.write_all(
        serde_json::to_string_pretty(object)
            .expect("File should be in json format")
            .as_bytes(),
    )
    .expect("Could not write to file");
}

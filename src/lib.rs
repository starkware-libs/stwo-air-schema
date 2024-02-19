#[cfg(test)]
use jsonschema::JSONSchema;
#[cfg(test)]
use std::fs;

#[cfg(test)]
fn json_from_file(path: &str) -> serde_json::Value {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(&contents).unwrap()
}

#[test]
fn example_matches_schema() {
    let schema_json = json_from_file("json/stwo_air.schema.json");
    let example_json = json_from_file("json/example_air.json");
    let schema = JSONSchema::options()
        .compile(&schema_json)
        .expect("The schema is invalid");
    assert!(
        schema.is_valid(&example_json),
        "The example JSON does not match the schema"
    );
}

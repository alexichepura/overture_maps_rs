use std::fs::{self, File};

// https://github.com/OvertureMaps/schema/blob/main/schema/schema.yaml
pub fn _schema_types() {
    // schemafy::schemafy!("../../overture_maps_schema/schema/schema.yaml");
    // schemafy::schemafy!("src/schema.json");
}

pub fn get_schema_json() {
    let path = format!("../overture_maps_schema/schema/schema.yaml");
    let file = File::open(path).expect("Unable to open file");
    let json_value: serde_json::Value = serde_yaml::from_reader(file).unwrap();
    let schema_string = serde_json::to_string(&json_value).expect("Schema string");
    fs::write("./schema/schema.json", schema_string).expect("Unable to write file");
    println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
}

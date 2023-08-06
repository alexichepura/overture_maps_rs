use std::fs::{self, File};
use walkdir::{DirEntry, WalkDir};

// https://github.com/OvertureMaps/schema/blob/main/schema/schema.yaml
pub fn _schema_types() {
    // schemafy::schemafy!("../../overture_maps_schema/schema/schema.yaml");
    // schemafy::schemafy!("src/schema.json");
}

pub fn get_schema_json() {
    let schema_path = "../overture_maps_schema/schema";
    let walker = WalkDir::new(&schema_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.expect("walkdir entry");
        let is_dir = entry.file_type().is_dir();
        let path = entry.path().display();
        let path_string = path.to_string();
        let path_relative = path_string.replace(schema_path, "");
        let name = entry.file_name();
        let name_str = name.to_str().expect("name_str");
        println!("{:?} {} {:?}", &is_dir, &path, &name_str);
        let rel = path_relative.replace(name_str, "");
        let json_name_str = name_str.replace(".yaml", ".json");
        let save_dir = format!("./schema{rel}");
        if is_dir {
            let save_dir_folder = format!("{save_dir}/{name_str}");
            dbg!(&save_dir_folder);
            fs::create_dir_all(&save_dir_folder).expect("created dir");
        } else {
            let schema_full_path = format!("{}{path_relative}", schema_path);
            let file = File::open(schema_full_path).expect("Unable to open file");
            let json_value: serde_json::Value = serde_yaml::from_reader(file).unwrap();
            let schema_string = serde_json::to_string_pretty(&json_value).expect("Schema string");
            // let schema_string = serde_json::to_string(&json_value).expect("Schema string");

            // let schema_string = schema_string.replace(".yaml", ".json");
            let save_path = format!("{save_dir}{json_name_str}");
            println!("{save_dir}:{save_path}");
            fs::write(save_path, schema_string).expect("Unable to write file");
        }
    }
    // let path = format!("../overture_maps_schema/schema/schema.yaml");
    // let file = File::open(path).expect("Unable to open file");
    // let json_value: serde_json::Value = serde_yaml::from_reader(file).unwrap();
    // let schema_string = serde_json::to_string(&json_value).expect("Schema string");
    // fs::write("./schema/schema.json", schema_string).expect("Unable to write file");
    // println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

use std::fs;
use yamson::json_to_yaml;

fn main() {
    // JSON to Yaml
    let input_filename = "sample.json";
    let output_filename = "output.yaml";

    let json_content = fs::read_to_string(input_filename).expect("Failed to read JSON file");

    json_to_yaml(&json_content, output_filename).expect("Failed to convert JSON to YAML");

    println!("JSON to YAML conversion successful");

    // Yaml to JSON
    let input_filename = "sample.yaml";
    let output_filename = "output.json";

    let yaml_content = fs::read_to_string(input_filename).expect("Failed to read YAML file");

    yamson::yaml_to_json(&yaml_content, output_filename).expect("Failed to convert YAML to JSON");

    println!("YAML to JSON conversion successful");
}

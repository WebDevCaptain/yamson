use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;

pub fn yaml_to_json(yaml_content: &str, out_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse
    let yaml_value: YamlValue = serde_yaml::from_str(yaml_content)?;

    // Serialize into JSON
    let json_content = serde_json::to_string_pretty(&yaml_value)?;

    // Write the JSON to output file
    fs::write(out_file, json_content)?;

    Ok(())
}

pub fn json_to_yaml(
    json_content: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse JSON
    let json_value: JsonValue = serde_json::from_str(json_content)?;

    // Serialize into YAML format
    let yaml_content = serde_yaml::to_string(&json_value)?;

    // Write the YAML output to file
    fs::write(output_file, yaml_content)?;

    Ok(())
}

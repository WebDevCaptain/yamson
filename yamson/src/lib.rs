use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;

/// Converts YAML content to JSON and writes it to a file
///
/// # Arguments
/// * `yaml_content` - YAML content to convert
/// * `out_file` - Output file to write the JSON content
///
/// # Example
/// ```rust
/// use std::fs;
/// use yamson::yaml_to_json;
///
/// let yaml_content = "name: Shreyash";
/// let out_file = "wdc.json";
///
/// yaml_to_json(yaml_content, out_file).unwrap();
///
/// // Check if the output file exists
/// assert_eq!(fs::metadata(out_file).is_ok(), true);
///
/// fs::remove_file(out_file).unwrap();
/// ```
///
/// # Errors
/// Returns an error if the YAML content is invalid or if the output file cannot be written to
pub fn yaml_to_json(yaml_content: &str, out_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse
    let yaml_value: YamlValue = serde_yaml::from_str(yaml_content)?;

    // Serialize into JSON
    let json_content = serde_json::to_string_pretty(&yaml_value)?;

    // Write the JSON to output file
    fs::write(out_file, json_content)?;

    Ok(())
}

/// Converts JSON content to YAML and writes it to an output file
///
/// # Arguments
/// * `json_content` - JSON content to convert
/// * `output_file` - Output file to write the YAML content
///
/// # Example
/// ```rust
/// use std::fs;
/// use yamson::json_to_yaml;
///
/// let json_content = r#"{"name": "Shreyash", "age": 49}"#;
/// let output_file = "wdc.yaml";
///
/// json_to_yaml(json_content, output_file).unwrap();
///
/// // Check if the output file exists
/// assert_eq!(fs::metadata(output_file).is_ok(), true);
///
/// fs::remove_file(output_file).unwrap();
/// ```
///
/// # Errors
/// Returns an error if the JSON content is invalid or if the output file cannot be written to
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

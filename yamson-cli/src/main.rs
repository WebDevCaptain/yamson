use std::{env, fs, process};

use yamson::{self, json_to_yaml, yaml_to_json};

fn main() {
    // Parse command line args
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        process::exit(1);
    }

    let output_file = &args[2];
    // Extract the input file extension
    let input_file = &args[1];
    let input_file_ext = input_file.split('.').last();

    let acceptable_input_file_extensions = ["yaml", "json", "yml"];

    if input_file_ext.is_none()
        || !acceptable_input_file_extensions.contains(&input_file_ext.unwrap())
    {
        eprintln!("Invalid input file: {}", input_file);
        process::exit(1);
    }

    let convert_from = input_file_ext.unwrap();

    // Read input file content
    let input_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file : {}", err);
            process::exit(1);
        }
    };

    // Perform conversion based on input file extension
    match convert_from {
        "json" => {
            if let Err(err) = json_to_yaml(&input_content, output_file) {
                eprintln!("Error converting to YAML: {}", err);
                process::exit(1);
            }
        }
        "yaml" | "yml" => {
            if let Err(err) = yaml_to_json(&input_content, output_file) {
                eprintln!("Error converting to JSON: {}", err);
                process::exit(1);
            }
        }
        _ => {
            eprintln!(
                "Invalid conversion type: '{}'. Use `json` or `yaml` ",
                convert_from
            );
            process::exit(1);
        }
    }

    println!("Done ✅");
}

# Yamson: JSON-YAML converter

[![CI](https://github.com/WebDevCaptain/yamson/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/WebDevCaptain/yamson/actions/workflows/ci.yml)

Yamson is structured as a workspace with `yamson` (core functionality) and `yamson-cli` (command line interface to test the core functionality).

## Features

1. Convert JSON to YAML
2. Convert YAML to JSON

## Usage (CLI)

```bash
cargo run -p yamson-cli <input_file> <output_file>
```

## Testing and Benchmarking

```bash
cargo bench --workspace --all-targets
```

```bash
cargo test --workspace --all-targets
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

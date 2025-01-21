use criterion::{criterion_group, criterion_main, Criterion};
use yamson::{json_to_yaml, yaml_to_json};

fn benchmark_json_to_yaml(c: &mut Criterion) {
    let json_content = r#"{"name": "Shreyash", "age": 49}"#;
    let output_file = "benchmark.yaml";

    c.bench_function("json_to_yaml", |b| {
        b.iter(|| json_to_yaml(json_content, output_file).unwrap())
    });

    std::fs::remove_file(output_file).unwrap();
}

fn benchmark_yaml_to_json(c: &mut Criterion) {
    let yaml_content = "name: Shreyash";
    let output_file = "benchmark.json";

    c.bench_function("yaml_to_json", |b| {
        b.iter(|| yaml_to_json(yaml_content, output_file).unwrap());
    });

    std::fs::remove_file(output_file).unwrap();
}

criterion_group!(benches, benchmark_json_to_yaml, benchmark_yaml_to_json);
criterion_main!(benches);

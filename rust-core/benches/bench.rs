use criterion::{black_box, criterion_group, criterion_main, Criterion};
use feature_probe_mobile_sdk_core::{FeatureProbe, Repository};
use serde_json::json;
use std::{fs, path::PathBuf};

fn bench_bool_toggle(fp: &FeatureProbe) {
    let _d = fp.bool_detail("bool_toogle", false);
}

fn bench_json_toggle(fp: &FeatureProbe) {
    let _d = fp.json_detail("multi_condition_toggle", json!(""));
}

fn load_json() -> Repository {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/fixtures/toggles.json");
    let json_str = fs::read_to_string(path).unwrap();
    serde_json::from_str(&json_str).unwrap()
}

//TODO: simulate repo read lock vs write lock with specific ratio
fn criterion_benchmark(c: &mut Criterion) {
    let repo = load_json();
    let fp = FeatureProbe::new_with(repo);

    c.bench_function("bench_bool_toggle", |b| {
        b.iter(|| bench_bool_toggle(black_box(&fp)))
    });

    c.bench_function("bench_json_toggle", |b| {
        b.iter(|| bench_json_toggle(black_box(&fp)))
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);

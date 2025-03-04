/* Copyright © 2025, CosmicMind, Inc. */

//! Benchmarks for the entid crate
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use entid::{Prefix, UlidEntityId, UuidEntityId};

// Define test entity types
struct User;
impl Prefix for User {
    fn prefix() -> &'static str {
        "user"
    }
}

struct Post;
impl Prefix for Post {
    fn prefix() -> &'static str {
        "post"
    }

    fn delimiter() -> &'static str {
        "-"
    }
}

fn bench_uuid_generation(c: &mut Criterion) {
    c.bench_function("uuid_generation", |b| {
        b.iter(|| {
            black_box(UuidEntityId::<User>::generate());
        })
    });
}

fn bench_ulid_generation(c: &mut Criterion) {
    c.bench_function("ulid_generation", |b| {
        b.iter(|| {
            black_box(UlidEntityId::<Post>::generate());
        })
    });
}

fn bench_uuid_parsing(c: &mut Criterion) {
    let id = UuidEntityId::<User>::generate();
    let id_str = id.as_str();

    c.bench_function("uuid_parsing", |b| {
        b.iter(|| {
            black_box(UuidEntityId::<User>::new(id_str).unwrap());
        })
    });
}

fn bench_ulid_parsing(c: &mut Criterion) {
    let id = UlidEntityId::<Post>::generate();
    let id_str = id.as_str();

    c.bench_function("ulid_parsing", |b| {
        b.iter(|| {
            black_box(UlidEntityId::<Post>::new(id_str).unwrap());
        })
    });
}

criterion_group!(
    benches,
    bench_uuid_generation,
    bench_ulid_generation,
    bench_uuid_parsing,
    bench_ulid_parsing
);
criterion_main!(benches);

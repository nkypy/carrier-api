extern crate carrier;
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_china_telecom(c: &mut Criterion) {
    use carrier::ChinaTelecomClient;

    c.bench_function("china telecom hash one value", |b| {
        let client = ChinaTelecomClient::new("test", "test", "abcdefghi");
        b.iter(|| client.hash(vec!["test"]))
    });
    c.bench_function("china telecom hash multi values", |b| {
        let client = ChinaTelecomClient::new("test", "test", "abcdefghi");
        b.iter(|| client.hash(vec!["14914000000", "test", "test", "queryPakage"]))
    });
}

criterion_group!(benches, bench_china_telecom);

criterion_main! {benches}

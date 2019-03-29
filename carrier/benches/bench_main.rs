extern crate carrier;
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

// use carrier::{ChinaTelecomClient, STATUS_NAME_HASHMAP};

fn bench_carrier(c: &mut Criterion) {
    use carrier::STATUS_NAME_HASHMAP;

    c.bench_function("carrier status_name_hashmap get ", |b| {
        b.iter(|| {
            STATUS_NAME_HASHMAP
                .get("china_telecom")
                .unwrap()
                .get("100001")
        })
    });
}

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

criterion_group!(benches, bench_carrier, bench_china_telecom);

criterion_main! {benches}

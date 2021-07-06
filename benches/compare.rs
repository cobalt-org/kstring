#![allow(
    clippy::clone_on_copy,
    clippy::identity_conversion,
    clippy::clone_double_ref
)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

type StringCow<'s> = std::borrow::Cow<'s, str>;

pub static FIXTURES: &[&str] = &[
    "",
    "0",
    "01",
    "012",
    "0123",
    "01234",
    "012345",
    "0123456",
    "01234567",
    "012345678",
    "0123456789",
    "01234567890123456789",
    "0123456789012345678901234567890123456789",
    "01234567890123456789012345678901234567890123456789012345678901234567890123456789",
];

fn bench_clone_static(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone static");
    for fixture in FIXTURES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("str", len), &len, |b, _| {
            let uut = *fixture;
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("StringCow", len), &len, |b, _| {
            let uut = StringCow::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KString", len), &len, |b, _| {
            let uut = kstring::KString::from_static(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KStringCow", len), &len, |b, _| {
            let uut = kstring::KStringCow::from_static(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KStringRef", len), &len, |b, _| {
            let uut = kstring::KStringRef::from_static(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
    }
    group.finish();
}

fn bench_clone_ref(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone ref");
    for fixture in FIXTURES {
        let len = fixture.len();
        let fixture = String::from(*fixture);
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("str", len), &len, |b, _| {
            let uut = fixture.as_str();
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(fixture.as_str());
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("StringCow", len), &len, |b, _| {
            let uut = StringCow::from(fixture.as_str());
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KString", len), &len, |b, _| {
            let uut = kstring::KString::from_ref(fixture.as_str());
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KStringCow", len), &len, |b, _| {
            let uut = kstring::KStringCow::from_ref(fixture.as_str());
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KStringRef", len), &len, |b, _| {
            let uut = kstring::KStringRef::from_ref(fixture.as_str());
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
    }
    group.finish();
}

fn bench_clone_owned(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone owned");
    for fixture in FIXTURES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = String::from(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("StringCow", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = StringCow::from(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KString", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = kstring::KString::from_string(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KStringCow", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = kstring::KStringCow::from_string(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
    }
    group.finish();
}

fn bench_eq_static(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq static");
    for fixture in FIXTURES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("str", len), &len, |b, _| {
            let uut = *fixture;
            let uut = criterion::black_box(uut);
            b.iter(|| uut == *fixture)
        });
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == *fixture)
        });
        group.bench_with_input(BenchmarkId::new("StringCow", len), &len, |b, _| {
            let uut = StringCow::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == *fixture)
        });
        group.bench_with_input(BenchmarkId::new("KString", len), &len, |b, _| {
            let uut = kstring::KString::from_static(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == *fixture)
        });
        group.bench_with_input(BenchmarkId::new("KStringCow", len), &len, |b, _| {
            let uut = kstring::KStringCow::from_static(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == *fixture)
        });
        group.bench_with_input(BenchmarkId::new("KStringRef", len), &len, |b, _| {
            let uut = kstring::KStringRef::from_static(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == *fixture)
        });
    }
    group.finish();
}

fn bench_eq_ref(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq ref");
    for fixture in FIXTURES {
        let len = fixture.len();
        let fixture = String::from(*fixture);
        let fixture = fixture.as_str();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("str", len), &len, |b, _| {
            let uut = fixture;
            let uut = criterion::black_box(uut);
            b.iter(|| uut)
        });
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
        group.bench_with_input(BenchmarkId::new("StringCow", len), &len, |b, _| {
            let uut = StringCow::from(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
        group.bench_with_input(BenchmarkId::new("KString", len), &len, |b, _| {
            let uut = kstring::KString::from_ref(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
        group.bench_with_input(BenchmarkId::new("KStringCow", len), &len, |b, _| {
            let uut = kstring::KStringCow::from_ref(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
        group.bench_with_input(BenchmarkId::new("KStringRef", len), &len, |b, _| {
            let uut = kstring::KStringRef::from_ref(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
    }
    group.finish();
}

fn bench_eq_owned(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq owned");
    for fixture in FIXTURES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = String::from(fixture.clone());
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
        group.bench_with_input(BenchmarkId::new("StringCow", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = StringCow::from(fixture.clone());
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
        group.bench_with_input(BenchmarkId::new("KString", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = kstring::KString::from_string(fixture.clone());
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
        group.bench_with_input(BenchmarkId::new("KStringCow", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = kstring::KStringCow::from_string(fixture.clone());
            let uut = criterion::black_box(uut);
            b.iter(|| uut == fixture)
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_clone_static,
    bench_clone_ref,
    bench_clone_owned,
    bench_eq_static,
    bench_eq_ref,
    bench_eq_owned,
);
criterion_main!(benches);

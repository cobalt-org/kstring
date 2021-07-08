#![allow(
    clippy::clone_on_copy,
    clippy::identity_conversion,
    clippy::clone_double_ref
)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

type StringCow<'s> = std::borrow::Cow<'s, str>;

pub static FIXTURES: &[&str] = &["0123456789", "0123456789012345678901234567890123456789"];

fn bench_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone");
    for fixture in FIXTURES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("&'static str", len), &len, |b, _| {
            let uut = *fixture;
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("Box<str>", len), &len, |b, _| {
            let uut = Box::<str>::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(
            BenchmarkId::new("StringCow::Borrowed", len),
            &len,
            |b, _| {
                let uut = StringCow::Borrowed(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
        group.bench_with_input(BenchmarkId::new("StringCow::Owned", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = StringCow::Owned(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(
            BenchmarkId::new("KString::from_static", len),
            &len,
            |b, _| {
                let uut = kstring::KString::from_static(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
        group.bench_with_input(BenchmarkId::new("KString::from_ref", len), &len, |b, _| {
            let fixture = String::from(*fixture);
            let uut = kstring::KString::from_ref(&fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(
            BenchmarkId::new("KString::from_string", len),
            &len,
            |b, _| {
                let fixture = String::from(*fixture);
                let uut = kstring::KString::from_string(fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
    }
    group.finish();
}

// Note: this is meant to measure the overhead for accessing the underlying str.  We shouldn't try
// to optimize *just* the case being measured here.
fn bench_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("access");
    for fixture in FIXTURES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("&'static str", len), &len, |b, _| {
            let uut = *fixture;
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(BenchmarkId::new("Box<str>", len), &len, |b, _| {
            let uut = Box::<str>::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(
            BenchmarkId::new("StringCow::Borrowed", len),
            &len,
            |b, _| {
                let uut = StringCow::Borrowed(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(BenchmarkId::new("StringCow::Owned", len), &len, |b, _| {
            let uut = StringCow::Owned(String::from(*fixture));
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(
            BenchmarkId::new("KString::from_static", len),
            &len,
            |b, _| {
                let uut = kstring::KString::from_static(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(BenchmarkId::new("KString::from_ref", len), &len, |b, _| {
            let uut = kstring::KString::from_ref(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(
            BenchmarkId::new("KString::from_string", len),
            &len,
            |b, _| {
                let uut = kstring::KString::from_string(String::from(*fixture));
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_clone, bench_access,);
criterion_main!(benches);

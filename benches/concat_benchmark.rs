use criterion::{Criterion, criterion_group, criterion_main};
use fast_cat::concat_str;
use std::hint::black_box;

fn small_concatenation_benchmark(c: &mut Criterion) {
    let s1 = "The quick brown fox ";
    let s2 = "jumps over ".to_string();
    let s3 = "the lazy dog. ";
    let s4 = "The quick brown fox ";
    let s5 = "jumps over ".to_string();
    let s6 = "the lazy dog.";

    let mut group = c.benchmark_group("Small Concatenation");

    group.bench_function("concat_str", |b| {
        b.iter(|| {
            black_box(concat_str!(s1, &s2, s3, s4, &s5, s6));
        })
    });

    group.bench_function("format", |b| {
        b.iter(|| {
            black_box(format!("{}{}{}{}{}{}", s1, s2, s3, s4, s5, s6));
        })
    });

    group.bench_function("addition", |b| {
        b.iter(|| {
            let mut res = s1.to_string();
            res = res + &s2 + s3 + s4 + &s5 + s6;
            black_box(res);
        })
    });

    group.bench_function("concat_slice", |b| {
        b.iter(|| {
            black_box([s1, &s2, s3, s4, &s5, s6].concat());
        })
    });

    group.bench_function("manual_with_capacity", |b| {
        b.iter(|| {
            let len = s1.len() + s2.len() + s3.len() + s4.len() + s5.len() + s6.len();
            let mut s = String::with_capacity(len);
            s.push_str(s1);
            s.push_str(&s2);
            s.push_str(s3);
            s.push_str(s4);
            s.push_str(&s5);
            s.push_str(s6);
            black_box(s);
        })
    });

    group.finish();
}

fn insert_benchmark(c: &mut Criterion) {
    let base_string = "Hello world!".to_string();
    let to_insert = ", beautiful";

    c.bench_function("String::insert_str", |b| {
        b.iter(|| {
            let mut s = base_string.clone();
            s.insert_str(5, black_box(to_insert));
        })
    });
}

criterion_group!(benches, small_concatenation_benchmark, insert_benchmark);

criterion_main!(benches);

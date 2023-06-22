#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn bench_slice_10(b: &mut Bencher) {
    b.iter(|| mult_persistence::calc_slice(10));
}

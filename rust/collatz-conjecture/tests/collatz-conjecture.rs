#![feature(test)]

extern crate test;

use collatz_conjecture::*;
use test::Bencher;

#[test]
fn test_1() {
    assert_eq!(Some(0), collatz(1));
}

#[test]
fn test_16() {
    assert_eq!(Some(4), collatz(16));
}

#[test]
fn test_12() {
    assert_eq!(Some(9), collatz(12));
}

#[test]
fn test_1000000() {
    assert_eq!(Some(152), collatz(1_000_000));
}

#[test]
fn test_0() {
    assert_eq!(None, collatz(0));
}

#[bench]
fn bench_test_1000000_while(b: &mut Bencher) {
    b.iter(|| assert_eq!(Some(152), collatz(1_000_000)));
}

#[bench]
fn bench_test_1000000_recurs(b: &mut Bencher) {
    b.iter(|| assert_eq!(Some(152), collatz_recurs(1_000_000)));
}

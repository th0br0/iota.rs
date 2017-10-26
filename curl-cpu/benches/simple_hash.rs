#![feature(test)]
#![feature(alloc)]
#![no_std]

extern crate alloc;
extern crate test;

extern crate iota_curl as curl;
extern crate iota_curl_cpu as cpucurl;
extern crate iota_trytes as trytes;

use test::Bencher;

use alloc::Vec;
use trytes::*;
use curl::*;
use cpucurl::*;

const TRINARY: &'static str = "9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               9999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999T999999999999999999999999999999\
                               99999999999999999999999OLOB99999999999999999999999";

#[bench]
fn curl_pair(b: &mut Bencher) {
    let _trits: Vec<Trit> = TRINARY.chars().flat_map(char_to_trits).cloned().collect();
    let trits: Vec<BCTrit> = _trits.into_iter().map(trit_to_bct).collect();

    let mut curl = CpuCurl::<BCTrit>::default();
    curl.reset();
    curl.absorb(&trits);

    let mut out = [(0, 0); HASH_LENGTH];
    curl.squeeze(&mut out);

    b.iter(|| {
        curl.reset();
        curl.absorb(&trits);
        curl.squeeze(&mut out);
    })
}

#[bench]
fn curl_simple(b: &mut Bencher) {
    let trits: Vec<Trit> = TRINARY.chars().flat_map(char_to_trits).cloned().collect();
    let mut curl = CpuCurl::<Trit>::default();
    let mut out = [0; HASH_LENGTH];
    curl.reset();
    curl.absorb(&trits);
    curl.squeeze(&mut out);

    b.iter(|| {
        curl.reset();
        curl.absorb(&trits);
        curl.squeeze(&mut out);
    })
}

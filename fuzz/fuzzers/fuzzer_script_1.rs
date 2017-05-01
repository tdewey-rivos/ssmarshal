#![feature(test)]

#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate ssmarshal;

extern crate test;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Simple {
    a: u8,
    b: (u16, u8),
    c: char,
    d: [usize; 3]
}

#[derive(Deserialize, Debug)]
enum Complex {
    A,
    B(Simple),
    C(u8, u16),
    D(isize),
    E {
        foo: Simple,
    },
    F {
        bar: Simple,
        qux: char,
        baz: Simple,
    }
}

fuzz_target!(|data: &[u8]| {
    match ssmarshal::deserialize::<Complex>(data) {
        Ok((val, bytes)) => { test::black_box(val); },
        Err(e) => { }
    }
});

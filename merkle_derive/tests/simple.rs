extern crate merkle;

#[macro_use]
extern crate merkle_derive;

use merkle::hash::Hashable;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[derive(Hashable, Debug)]
struct Foo {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: String,
    f: &'static str,
}

#[test]
fn test_foo_hash() {
    let foo = Foo {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: String::from("bar"),
        f: "bar",
    };

    let hr = &mut DefaultHasher::new();
    foo.hash_state(hr);
    assert_eq!(hr.finish(), 8466196983143881409)
}
// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;
}

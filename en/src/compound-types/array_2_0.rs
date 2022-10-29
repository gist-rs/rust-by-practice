fn main() {
    // we can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Arrays are stack allocated, `std::mem::size_of_val` return the bytes which array occupies
    // A char takes 4 byte in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);
}

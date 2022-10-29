fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // you can't use r1 and r2 at the same time
    r1.push_str("world");
}

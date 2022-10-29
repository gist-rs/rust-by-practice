fn main() {
    let mut s = String::from("hello, ");

    // fill the blank to make it work
    let p = &mut s;

    p.push_str("world");
}

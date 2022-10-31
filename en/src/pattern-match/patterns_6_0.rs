fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        // The type of value is &mut String
        value => value.push_str(" world!"),
    }
}

fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    borrow_string(&s);

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn borrow_string(s: &str) {
    println!("ownership of \"{}\" is still with the variable 's', only the reference is passed", s)
}

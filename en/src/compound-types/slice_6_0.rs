fn main() {
    let mut s = String::from("hello world");

    // here, &s is `&String` type, but `first_word` need a `&str` type.
    // it works because `&String` can be implicitly converted to `&str, If you want know more ,this is called `Deref`
    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    &s[..1]
}

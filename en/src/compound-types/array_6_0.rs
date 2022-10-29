fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // but indexing is not safe
    let _name1 = &names[1];
}

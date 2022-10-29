fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit())
}

fn implicitly_ret_unit() {
    println!("I will returen a ()")
}

// don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will returen a ()")
}

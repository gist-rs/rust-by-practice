use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    // unit type does't occupy any memeory space
    assert!(size_of_val(&unit) == 0);
}

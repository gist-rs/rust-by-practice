fn main() {
    let x: i32 = 5; // uninitialized but using, ERROR !
    let y: i32; // uninitialized but also unusing, only warning
    println!("{} is equal to 5", x);
}

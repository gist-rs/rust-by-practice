struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T,
}
fn main() {
    println!("Success!")
}

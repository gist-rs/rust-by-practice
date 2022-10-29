fn my_function<const N: usize>() -> [u32; N] {
    [123; N]
}

fn main() {
    let arr = my_function::<5>();
    println!("{:?}", arr);
}

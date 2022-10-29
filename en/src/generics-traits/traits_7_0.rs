fn main() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.0, 2.0), 3.0);
}

fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

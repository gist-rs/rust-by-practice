fn main() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.0, 2.0), 3.0);
}

fn sum<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    x + y
}

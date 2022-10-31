fn main() {
    let x = Box::new(5);

    let mut y = Box::new(3); // implement this line, dont change other lines!

    *y = 4;

    assert_eq!(*x, 5);
}

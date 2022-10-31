fn main() {
    let mut v = Vec::new();
    for n in 1..101 {
        v.push(n);
    }

    assert_eq!(v.len(), 100);
}

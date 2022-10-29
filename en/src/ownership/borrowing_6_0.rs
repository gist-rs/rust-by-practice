fn main() {
    let c = '中';

    let r1 = &c;
    // fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));
}

// get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

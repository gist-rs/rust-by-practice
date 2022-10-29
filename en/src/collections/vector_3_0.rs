fn main() {
    // array -> Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);


    // String -> Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    println!("Success!")
 }

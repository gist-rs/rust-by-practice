
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // fill in the blank to let p match the second arm
    let p = Point { x: 2, y: 20 }; // x can be [0, 5], y can be 10 20 or 30

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

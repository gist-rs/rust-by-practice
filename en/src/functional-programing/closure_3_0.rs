fn main() {
     // A non-copy type.
     let movable = Box::new(3);

     // A copy type would copy into the closure leaving the original untouched.
     // A non-copy must move and so `movable` immediately moves into
     // the closure.
     let consume = || {
         println!("`movable`: {:?}", movable);
         take(movable);
     };

     consume();
    // consume();
}

fn take<T>(_v: T) {

}

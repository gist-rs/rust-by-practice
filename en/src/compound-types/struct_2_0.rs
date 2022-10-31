struct Unit;
trait SomeTrait {
    // ...Some behavours defines here
}

// We don't care the the fields are in Unit, but we care its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {}
fn main() {
    let u = Unit;
    do_something_with_unit(u);
}

// Fill the blank to make the code work
fn do_something_with_unit(u: Unit) {}

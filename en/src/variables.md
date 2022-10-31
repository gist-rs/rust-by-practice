# Variables

### Binding and mutability

1. 🌟 A variable can be used only if it has been initialized.

```rust,editable
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
```

{{#playground variables_1_0.rs answer}}

2. 🌟 Use `mut` to mark a variable as mutable.

```rust,editable
// Fill the blanks in the code to make it compile
fn main() {
    let __ = 1;
    __ += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
```

{{#playground variables_2_0.rs answer}}

### Scope

A scope is the range within the program for which the item is valid.

3. 🌟

```rust,editable
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}
```

{{#playground variables_3_0.rs answer}}

4. 🌟🌟

```rust,editable
// Fix the error with the use of define_x
fn main() {
    println!("{}, world", x);
}

fn define_x() {
    let x = "hello";
}
```

{{#playground variables_4_1.rs answer}}
{{#playground variables_4_0.rs answer}}

### Shadowing

You can declare a new variable with the same name as a previous variable, here we can say \*\*the first one is shadowed by the second one.

5. 🌟🌟

```rust,editable
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // Prints "42".
}
```

{{#playground variables_5_0.rs answer}}

6. 🌟🌟

```rust,editable
// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}
```

{{#playground variables_6_0.rs answer}}

### Unused variables

7. Fix the warning below with :

- 🌟 Only one solution
- 🌟🌟 Two distinct solutions

> Note: none of the solutions is to remove the line `let x = 1`

```rust,editable
fn main() {
    let x = 1;
}

// Warning: unused variable: `x`
```

{{#playground variables_7_0.rs answer}}
{{#playground variables_7_1.rs answer}}

### Destructuring

8. 🌟🌟 We can use a pattern with `let` to destructure a tuple to separate variables.

> Tips: you can use Shadowing or Mutability

```rust,editable
// Fix the error below with least amount of modification
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
```

{{#playground variables_8_1.rs answer}}
{{#playground variables_8_0.rs answer}}

### Destructuring assignments

Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

9. 🌟🌟

> Note: the feature `Destructuring assignments` need 1.59 or higher Rust version

```rust,editable
fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], __);

    println!("Success!");
}
```

{{#playground variables_9_0.rs answer}}

> You can find the solutions [here](https://github.com/sunface/rust-by-practice)(under the solutions path), but only use it when you need it

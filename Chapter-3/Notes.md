# Chapter 3 Notes

*This page essentially contains notes that I took to summerise the contents of Chapter 3 from the book*

- [Chapter 3 Notes](#chapter-3-notes)
  - [Variable and Mutability](#variable-and-mutability)
    - [Constants](#constants)
    - [Shadowing](#shadowing)
  - [Data Type](#data-type)


## Variable and Mutability

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

`let x = 5;` is an immutable variable, meaning that we can't reassign a new values after assignment, therefore `x=6;` results in an error.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

To overcome this we would have to make the variable mutable with `mut`.

### Constants

They are values that can't be changes once assigned, but are different from variables.

1. You can't use `mut` with constants, they are immutable by default.
2. Constants are declared using `const` and the values must be *annotated*.
3. They can be declared in any scope
4. They can't be set by the result of a value that would be computed at runtime, so they can only be set to a constant expression

Example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Rust's naming convention for constants is to be **all upper case and snake case**.

They are valid for the entire time a program runs, within the scope that they are declared.

### Shadowing

You are able to declare a new variable with a new variable with the same name as a previous variable. This is what is referred to as shadowing the first variable with the second.

The shadowed version is only valid until the it either shadows itself OR the scope of the shadowed variable ends.

An Example would be:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```

The output of the Example:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

Another difference between `mut` and shadowing is that we are effectively creating a new variable when we use the `let` keyword again, so we can change the types of the values but reuse the same name.

Example:

```rust
    let spaces = "   ";
    let spaces = spaces.len();
```

However you can't use this for mutated variable.

- *Basically, can't change the type of data store in a mutable variable*

## Data Type

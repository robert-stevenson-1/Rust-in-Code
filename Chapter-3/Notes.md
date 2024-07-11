# Chapter 3 Notes

*This page essentially contains notes that I took to summerise the contents of Chapter 3 from the book*

- [Chapter 3 Notes](#chapter-3-notes)
  - [Variable and Mutability](#variable-and-mutability)
    - [Constants](#constants)
    - [Shadowing](#shadowing)
  - [Data Type](#data-type)
    - [Integer](#integer)
    - [Float-Point](#float-point)
    - [Numeric Operation](#numeric-operation)
    - [Boolean](#boolean)
    - [Character](#character)
  - [Compound Types](#compound-types)
    - [Tuple](#tuple)
    - [Array](#array)
  - [Functions](#functions)
    - [Parameters](#parameters)
    - [Statements and Expressions](#statements-and-expressions)


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

They are values that can't be changed once assigned, but are different from variables.

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

Scalar Types:

- Represent  a single value
- 4 Primary types:
  - integer
  - floating-point
  - numbers
  - Booleans
  - character

### Integer

| Length  | Signed | Unsigned |
| :------ | :----- | :------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

### Float-Point

They have two primary types:
- `f32`, 32-bit
- `f64`, 64-bit
  
### Numeric Operation

basic mathematical operations for all the number types:

- addition
- subtraction
- multiplication
- division
- remainder

### Boolean

*Just like any other language really...*

### Character

Rust’s char type is four bytes in size and represents a Unicode Scalar Value, meaning it can represent more than just ASCII.

## Compound Types

### Tuple

- A general way of grouping together values of various types
- Mixed length
- Cannot grow or shrink once declared
- When accessing a tuple element directly we have to use a `.` to index, eg `let x = (12, 3); let x1 = x.0;`

### Array

- Array are fixed length
- Every element is the same type
- Store data on the stack instead of the heap (See [Chapter 4 Notes](/Chapter-4/Notes.md))
- *vector* is similar collection that allows for the growth and shrinking of the length
- used square bracket notation `[el_1, el_2, el_3, ..., el_N]`

access element notation

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

## Functions

- `main` is the entry point of the program.
- `fn` is used to declare the function

example:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

-  Rust doesn't care about where the function is declared
   -  Function just have to be in scope with where they are being called

### Parameters

- parameters are declared with the data type specified

example:
```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### Statements and Expressions

- **Statements** are instructions that perform some sort of action and done
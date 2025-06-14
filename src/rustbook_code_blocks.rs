// string constants for side_nav popup panels

// const for Variable, Mutability, & DataTypes popup
pub const VARIABLES_MUTABILITY_DATATYPES: &str = r#"
//Variables, Mutability & DataTypes in Rust

// This code would not compile, because by default, variables are immutable, and as the RustBook states, 'When a variable is immutable, once a value is bound to a name, you cant change that value.'
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

Good code that would compile bc value is declared mutable,

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// ahhh Data Types in Rust, they are beautiful and we know exactly what the are, always. That feels good doesnt it?

Look at two data types in Rust: Scalar and compound

Scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. similar to other programming languages.

// Integers

Integers are numbers without a fractional component. It Chapter 2 of the Rust book we used the u32 type. The u32 type indicates that the value associated with it should be an unsigned integer that takes up 32 bits of space. Signed or unsigned refer to whether the number can be negative, i.e. if the number needs to have a sign(signed), or if the number can only ever be positive(unsigned).

// Floating Point Types

Rust has two primitive types for floating-point numbers(numbers with decimals).

The types are f32 and f64, 32 bits and 64 bits respectively.

Default type is f64 bc on modern CPUs it about the same speed as f32 but capable of more precisness. All floating-point types are signed.

Here are floating point numbers doing their thing,

fn main() {
  let x = 9.0; // f64
  let y: f32 = 6.0 // f32
}

// Numeric Operations

Rust supports basic arithmetic: addition (+), subtraction (-), multiplication (\*), division (/), and remainder (%).
Integer division truncates toward zero. Example:

let sum = 5 + 10;
let remainder = 43 % 5;
____________________
// Boolean Type

Rust’s `bool` type represents either `true` or `false` and is 1 byte in size.
Used mainly in conditional logic:

let t = true;
let f: bool = false;
____________________
// Character Type

Rust’s `char` type represents a Unicode Scalar Value (4 bytes).
It can store a wide range of characters, including emojis and non-Latin scripts.
Character literals use single quotes:

let c = 'z';
let emoji = '😻';
_____________________
// Compound Types

// Tuples

Tuples group multiple values of different types into a single, fixed-size structure:

let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let first = tup.0;

The unit type `()` is an empty tuple used for functions that return no value.
_____________________
// Arrays

Arrays store multiple values of the same type with a fixed length:

let a = [1, 2, 3, 4, 5];
let months = ["January", "February", ..., "December"];
let repeated = [3; 5]; // [3, 3, 3, 3, 3]

Access elements using indices:

let first = a[0];
let second = a[1];

Accessing an out-of-bounds index causes a runtime panic to maintain memory safety.
"#;

// string constants for side_nav popup panels

// const for Variable, Mutability, & DataTypes popup
pub const VARIABLES_MUTABILITY_DATATYPES: &str = r#"
Variables, Mutability & DataTypes in Rust

// This code would not compile, because by default, variables are immutable, and as the RustBook states, 'When a variable is immutable, once a value is bound to a name, you cant change that value.'

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// In the provided Rust example, a compilation error would occur because the code attempts to assign a new value to an 'immutable variable `x`', which is not allowed in Rust by default.

// Error Message

$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error

// This happens because `x` was declared with `let x = 5;`, making it immutable, and then reassigned later with `x = 6;`.

// Solution:

To make the variable mutable, use `let mut x = 5;`.

// Good code that would compile bc 'x' is declared mutable,

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6

Why This Matters:

  * Rust enforces `immutability by default` to prevent bugs that arise from unexpected value changes.
  * This makes code more predictable and easier to debug, especially in concurrent or complex systems.
  * If you need to change a variable, explicitly marking it as mutable (`mut`) signals that intent to other developers.

// Takeaway:
// Compiler errors in Rust are designed to help you write **safe and correct code**. Even experienced developers encounter them — they’re part of the learning process and a core strength of Rust's safety guarantees.

**Constants**

// Constants are values bound to a name and are not allowed to change, similar to immutable variables. There are differences though between constants and variables, so lets take a look.

🧱 Key Characteristics

| Feature             | Constants                         | Variables                |
| ------------------- | --------------------------------- | ------------------------ |
| Mutability          | Always immutable                  | Immutable by default     |
| Declaration Keyword | `const`                           | `let` or `let mut`       |
| Type Annotation     | **Required**                      | **Required**             |
| Scope               | Any (including global scope)      | Typically local scope    |
| Value Requirement   | Must be a **constant expression** | Can be a runtime value   |

Example:

const FERRIS_APPROVAL_RATING: u8 = 100;

**Shadowing**

// Shadowing allows you to redeclare a variable with the same name, effectively "shadowing" the previous one.
// It can even change the **type** or **mutability**.

let z = 5;
let z = z + 1; // shadows the previous z
let z = z * 2; // shadows again
println!("The value of z is: {}", z); // prints 12

let spaces = "   ";
let spaces = spaces.len(); // `spaces` now holds an integer

// Unlike `mut`, shadowing creates a **new variable**, which is useful for transformations.

Summary:

| Concept     | Keyword     | Mutable by Default | Can Change Type | Redeclaring Allowed     |
|-------------|-------------|--------------------|-----------------|-------------------------|
| Variable    | `let`       | ❌ No              | ❌ No          | ✅ With shadowing       |
| Mutable Var | `let mut`   | ✅ Yes             | ❌ No          | ❌ No                   |
| Constant    | `const`     | ❌ No              | ❌ No          | ❌ No                   |
| Shadowing   | `let` again | ✅ Yes             | ✅ Yes         | ✅ (via new declaration)|

___________________________________________________________________________________
Data Types

// in Rust, Data Types are beautiful as we know exactly what the are, always. That feels good doesnt it? Every value in Rust has a data type. Rust is a statically type coding language and must know data types at compile time.

let number_guess: u32 = "42".parse().expect("Not a number!");

if the `: u32` type annotation is not added to the code above, the compiler would throw an error, `error[E0284]: type annotations needed`

// Look at two data types in Rust: Scalar and compound

// Scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. similar to other programming languages.

**Integers**

// Integers are numbers without a fractional component. It Chapter 2 of the Rust book we used the u32 type. The u32 type indicates that the value associated with it should be an unsigned integer that takes up 32 bits of space. Signed or unsigned refer to whether the number can be negative, i.e. if the number needs to have a sign(signed), or if the number can only ever be positive(unsigned).

let number_guess: u32 = "42".parse().expect("Not a number!");

Floating Point Types

// Rust has two primitive types for floating-point numbers(numbers with decimals).

// The types are f32 and f64, 32 bits and 64 bits respectively.

// Default type is f64 bc on modern CPUs it about the same speed as f32 but capable of more precisness. All floating-point types are signed.

// Here are floating point numbers doing their thing,

fn main() {
  let x = 9.0; // f64
  let y: f32 = 6.0 // f32
}


**Numeric Operations**

// Rust supports basic arithmetic: addition (+), subtraction (-), multiplication (\*), division (/), and remainder (%).
// Integer division truncates toward zero to the nearest integer.

Example:

let sum = 5 + 10;
let remainder = 43 % 5;

**Boolean Type**

Rust’s `bool` type represents either `true` or `false` and is 1 byte in size.
Used mainly in conditional logic:

let t = true;
let f: bool = false;

**Character Type**

Rust’s `char` type represents a Unicode Scalar Value (4 bytes).
It can store a wide range of characters, including emojis and non-Latin scripts.
Character literals use single quotes:

let c = 'z';
let emoji = '😻';
_______________________________________________________________________________________
Compound Types

**Tuples**

Tuples group multiple values of different types into a single, fixed-size structure:

let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let first = tup.0;

The unit type `()` is an empty tuple used for functions that return no value.

**Arrays**

Arrays store multiple values of the same type with a fixed length:

let a = [1, 2, 3, 4, 5];
let months = ["January", "February", ..., "December"];
let repeated = [3; 5]; // [3, 3, 3, 3, 3]

Access elements using indices:

let first = a[0];
let second = a[1];

Accessing an out-of-bounds index causes a runtime panic to maintain memory safety.
"#;

// const for Variable, Mutability, & DataTypes popup
pub const FUNCTIONS: &str = r#"
// Functions

Functions are used prevalently in Rust.

There is one of the most important funtions, the 'main' function, the entry point of many programs.

// ================================================
// 1. Function Basics
// ================================================
// For function and variable names, RUST uses snake case, where all letters are lowercase and underscores seperate words. example_snake_case
// Functions are defined by the use of the fn keyword, followed by a function name and a set of parentheses, then a set of curly brackets to tell the compiler where the function body begins and ends.

fn basic_function() {
    // This function takes no arguments and returns nothing
    println!("This is a basic function call.");
}

// ================================================
// 2. Defining and Calling Functions
// ================================================

// Any function can then be called by entering its name followed by a set of parentheses.
// Functions can be defined before or after your main function, as Rust does not care where you define your functions, you just need to be sure they are defined somewhere in a scope that can be seen by the caller.

fn main() {
    println!("Welcome to the Rust functions demo!");

    // Calling a basic function
    basic_function();

    // Calling a function with parameters
    print_measurement(5, 'h');

    // Calling a function that returns a value
    let result = plus_one(5);
    println!("5 + 1 = {}", result);
}

// ================================================
// 3. Function Parameters
// ================================================
// Functions in Rust can be defined to have parameters. Parameters in functions are special variables that are part of a functions signature. When parameters are defined for a function, you can pass it values for those parameters. Values passed into function parameters are also called arguments. But the words parameter and argument are often used interchangeably in casual conversation to describe the variables in a functions definition or the values passed in when a function is called.

// The type of each parameter is required to be declared in function signatures. This is deliberate by design in Rust. To determine what type you are using, The compiler almost never needs you to use them elsewhere, since the type annotations are required in function definitions.

// Multiple parameters are defined by separating parameter declarations with commas.

fn print_measurement(value: i32, unit_label: char) {
    // Parameters must have types explicitly declared
    println!("The measurement is: {}{}", value, unit_label);
}

// ================================================
// 4. Function Return Values
// ================================================
// Functions can return values with the `->` syntax, followed by the type of the return value.

fn plus_one(x: i32) -> i32 {
    // Implicit return: no semicolon means the value is returned
    x + 1
    // If you put a semicolon here, it becomes a statement, not an expression, and the function will return `()`
}

// ================================================
// 5. Statements vs Expressions
// ================================================
// - Statements: instructions that perform actions and do not return a value
// - Expressions: evaluate to a value
// A series of statements optionally ending in an expression are what make up Function bodies. Rust is an expression based language and this is important to understand, as other languages don't have that same distinction.

// Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value

// Function definitions are statements. 

// Statements do not have return values.

// For example, let y = 6 is a statement as it does not have a return value. This in not like other languages where the assignment returns the value of the assignment, such as in C or Ruby.

// Another example:
fn _statement_vs_expression() {
    let y = {
        let x = 3;
        x + 1 // This is an expression, the value of this block is 4
    };
    println!("y = {}", y);
}

// ================================================
// 6. Practice: Writing Your Own Functions
// ================================================
// Challenge yourself to:
// - Write a function that takes 3 parameters
// - Return a computed result

// Example:
fn multiply_add(a: i32, b: i32, c: i32) -> i32 {
    (a * b) + c
}

// ================================================
// 7. Recap
// ================================================
// - `fn` to define functions
// - Parameters require explicit types
// - Use `->` to specify a return type
// - Expression without semicolon for return
// - Function calls must match parameter types
"#;

pub const COMMENTS: &str = r#"  
// Demonstrates comments in Rust as described in Chapter 3.4 of The Rust Book.

// Comments are used to explain code, making it easier to understand.

fn main() {
    // Simple line comment
    // This is a single-line comment in Rust.
    // It starts with `//` and continues to the end of the line.

    // Multi-line comment using repeated `//`:
    // Sometimes you want to write a longer explanation.
    // It's idiomatic in Rust to use multiple single-line comments for this.

    /*
        Block comment:
        These are less common but can span multiple lines
        without needing to start each line with `//`.
    */

    let x = 5; // Inline comment: This sets x to 5

    let y = x + 1; // Adds 1 to x and stores it in y

    println!("x = {}, y = {}", x, y); // Prints the values
}
"#;

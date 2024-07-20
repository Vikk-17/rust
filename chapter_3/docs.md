# Variables and Mutability
- By default, variables are immutable. This is one of many nudges Rust gives us to write our code in a way that take advantage of the safety and easy concurrency that Rust offers.

### Constants
- Like immutable variables, Constants are values that are bound to a name and are not allowed to change.
- First, we are not allowed to use `mut` with constants. Constants aren't just immutable by default -- they are always immutable.
- We declare constants using the `const` keyword instead of `let` keyword, and the type of the value must be annotated.
- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
- The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

`const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

### Shadowing
- We can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when we use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
- We can shadow a variable by using the same variable's name and repeating the use of the `let` keyword.
- Shadowing is different from marking a variable as `mut` because we'll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
- The other difference between `mut` and shadowing is that because we're effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.

`
let spaces = "    "; // strings
let spaces = spaces.len() // numbers;
`
`
let mut spaces = "    ";
spaces = spaces.len(); // not allowed to mutate a variable's type.
`

# Data Types
- Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how use it.
- In cases when many types are possible, such as converting a `String` to a numeric type using `parse`, we must add a type annotation, like this:

`
let guess: u32 = "42".parse().expect("Not a number!");`
`

## Scalar Types
- Rust has four primary scalar types: integers, floating point, Booleans, and characters.

### Integer Types 

| Length | Signed | Unsigned |
| ------ | -------| -------- |
| 8-bit  |  i8    |    u8    |
| 16-bit |  i16   |    u16   |
| 32-bit |  i32   |    u32   |
| 64-bit |  i64   |    u64   |
| 128-bit|  i128  |    u128  |
| arch   |  isize |    usize |
| ------ | ------ | -------- |

- Signed numbers are stored using two's complement representation.
- Range of signed numbers `-(2^(n-1)) to 2^(n-1) inclusive, n = number of bits`
- Additionally, the `isize and usize` types depend on the architecture of the computer we are running on.

|    Number Literals   |    Example    |
|    ---------------   |  -----------  |
|       Decimal        |98_222 => 98222|
|       Hex            |     0xff      |
|       Octal          |     0o77      |
|       Binary         |  0b1111_0000  |
|       Byte(u8 only)  |     b'A'      |
|   -----------------  |  -----------  |

- Rust's defaults are generally good places to start: integer types default to `i32`.

### Floating-Point Types 
- Floating-Point types are `f32 (single precision) and f64 (double precision)`
- The default type is `f64` because on modern CPUs, it's roughly the same speed as `f32` but is capable of more precision
- All floating-point types are Signed

### Boolean Type 

```
fn main() {
  let t = true;
  let f: bool = false; // with explicit type annotation
}
```

### Character Type 

```
fn main(){
  let c = 'z';
  let z: char = 'Z'; // with explicit type annotation
  let heart_eyed_cat = '😻';
}
```
- `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
- Unicode Scalar Values range from `U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive`


## Compound Types
- Compound types can group multiple values into one type 
- tuples and arrays

### The Tuple Type 
- Has fixed length, once declared, they can't grow or shrink in size

```
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

- The variable `tup` binds the entire tuple because a tuple is considered a single compound element.

```
fn main() {
  let tup = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!(The value of y is: {y});
}
```
- accessing tuple => `tup.0 = 500, tup.1= 6.4, tup.2=1`
- The tuple without any values is called `unit`.
- The value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expression implicitly return the unit value if they don't return any other value.

### The Array Type 

```
fn main() {
  let a = [1, 2, 3];
  let b: [i32; 3] = [1, 2, 3]; // 3, i32 integers
  let c = [3; 5] // c = [3, 3, 3, 3, 3]
  // access using index b[0]
}
```

### Statements and Expression
- Statements: instructions that perform some action and do not return a value;
- Expression: evaluate to resultant value.

- `let y = 6;`  is an Statement.
- Function definitions are also Statements;
- `let x = (let y = 6); // Error`
- `let y = 6` statement does not return a value, so there is not anything for x to bind to. In other languages such as C and Ruby, where the assignment returns the value of the assignment. So, `x = y = 6` is possible but not in case of Rust.
- Expression can be a part of the statement.
- Calling a Function is an expression, calling a micro is an expression, a new scope block created with curly brackets is an expression

```
fn main() {
  let y = {    
    let x = 3;
    x + 1
  };
  println!("The value of y is: {y}");
}

/* This expression:
  {
    let x = 3;
    x + 1
  }
*/
```

### Function with return value:
- We don't name return values, but we must declare their type after an arrow `(->)`.
- In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
- We can return early from a function by using the `return` keyword and specifying the value, but most functions return the last expression implicitly.

```
fn five() -> i32 {
  5 // with no semicolon because it's an expression whose value we want to return
}
fn main() {
  let x = five();
  println!("The value of x is: {x}");
}
```

```
fn main() {
  let x = plus_one(5);
  println!("The value of x is: {x}");
}
fn plus_one(x:i32) -> i32 {
  x + 1 
}
```


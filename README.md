
# Rustaceans
Notes from each chapter of the rust lang tutorial.

## Chapter 1
Important commands:
```
cargo new <project name>
	- creates a new project
cargo run
	- build the project and also runs
cargo build
	- build the project to ./target/debug/<name>
cargo check
	- checks code but doesn't product an executable
cargo build --release

```

### Cargo.toml
- declare packages here 
- then run cargo build, this will install the package
- cargo upadte to update packages

## Chapter 2

### User Input
```rust
use  std::io;

io::stdin().read_line(&mut variable)
```

### Match
```rust
match num.cmp(&other_num) {
	Ordering::Less => do something,
	Ordering::Greater => do something else,
	Ordering::Equal => 	
}
```

## Chapter 3

### Const Variables
declare a variable as const with `const var: type = ...;`
- always have to give its type when using const
Example: `const MAX_POINTS: u32 = 100_000;` 

### Shadowing
A variable cannot be mutated unless explicity declared as so, but you case reuse the same variable in shadowing
Example:
```rust
let x = 5;

x = 6 // this will fail to compile

let x = 5;

let x = x + 1;

let x = x * 2; // this will compile successfully  
```

Also with shadowing the variable can change type.

```rust
let x = "   ";
let x = x.len();

```
### Functions
Even though rust is an expression based language, statements are also common (don't return any value).

Example: `let y = (let x = 5);` is invalid;

```rust
let y = {
    let x = 5; 
    x + 1
}; // this is valid: note the lack of semi-colon after x + 1.
```

```rust
let y = five();

fn five() -> u32 {
    5
} // again note the lack of semi-colon.
```

### Control Flow

If expressions must contain type bool in their guard

```rust
let number = 3;

// this will fail to compile
if number {
    println!("Number is: {}", number);
} 
```
Because these are expressions, they return a value and therefore can be using with a let statement.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
// number will get the value 5
```

## Chapter 4

Ownership

### Move

```rust
let s1 = String:from("Hello world"); // s1 live on stack but points to memory on the heap that contains the string
let s2 = s1; // this is valid but the heap block is now owned by s2, accessing s1 is an error.

println!("{}", s1); // wont compile
```
### Copy
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens. 
```

## Chapter 9

Error handling.

In rust there are two type of errors:
1. recoverable in the form of `Result<T, E>`, for example file not found error where you want to continue a program.
2. unrecoverable in the form of the `panic!` macro which halts a program.

### panic!
- program will print a failure message
- unwind
- clean up the stack
- quit

Example:
```rust
fn main() {
	panic!("program crashed");
}
---------------------------------
fn main() {
	let v = vec![1, 2, 3];
	v[99];
}
```

if we run the program using the command:
```shell
RUST_BACKTRACE=1 cargo run
```
we can see a more detailed backtrace

### Result
```rust
enum Result<T, E> { 
    Ok(T),
    Err(E),
}
```
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```
Matching on different kind of errors.
```rust
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.text") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => { 
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```
or
```rust
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("File could not be created: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
   });
}
```

### Propogation shortcut
If wanting to continue with our code or we intend to return the error if it arises,
we can use the ? operator. Note that ? is only valid inside a function that returns
Result.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
```rust
use std::error:Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn, Error>> {
   let f = File::open("hello.txt")?;
   Ok(())
}
```

## Chapter 10 

Generics

We can create generic functions, enums, traits and add lifetimes.

We can also implement functions on a generic that can only be called by certain types.
For example a `Point<T>` enum, and
```rust
impl Point<i32>
    fn dist(&self) -> i32 {
          self.x
     }
}
```

Rust implements generics in such a way that your code doesn’t run any slower using generic types
than it would with concrete types. When Rust compiles code, it performs monomorphization. 
During that process, the compiler reads the values that have been used in the generic and
identifies the types. 
For example: `Some(5) and Some(5.0) will be expanded into Option_i32 and Options_f32`.

### Traits
Similar to interfaces in other languages, with some differences.

```rust
pub trait Printable {
    fn to_str(&self) -> String;
}
```

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Printable for NewsArticle {
    fn to_str(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```
Default Trait and traits as paramets

```rust 
pub trait Printable {
    fn to_str(&self) -> String {
         format!("{} by {}", self.headline, self.author)
    }
}

impl Printable for NewsArticle {}

fn notify(item: &impl Printable) {
    println!("Breaking news: {}", item.to_str());
}

```

Generic and multiple trait bounds

```rust
fn notify<T: Printable>(item: &T) {
    println!("Breaking news: {}", item.to_str());
}

fn notify<T: Printable + Display>(item: &T) 

fn notify(item: &(impl Printable + Display))
```

Traits with the where clause

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> String {
...
}

vs

fn some_function<T, U>(t: &T, u: &U) -> String
    where T: Display + Clone,
          U: Clone + Debug
{
...
}
```

Returning Types that implement traits

```rust
fn returns_printable() -> impl Printable {
...
}
```

Implement a function for a struct that implements other traits. The below code
implements the function display largest for any Pair that implements the traits
Dsiplay and PartialOrd

```rust
impl<T: Display + PartialOrd> Pair<T> {
     fn display_largest(&self) {
          if self.x > self.y {
               println!("x = {}", self.x);
          } else {
               println!("y = {}", self.y);
          }
      }
}
```

Blanket Implementations.
The below code implements ToString trait on any type that implements `Display` trait.

```rust
impl <T: Display> ToString for T {
   ...
}
```

#### Lifetime Annotation
Lifetime annotations don't change how long any of the references live, they are there to describe the relationships of the
lifetimes of multiple references without affecting the lifetimes.

```rust
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a i32 // a mutable reference with an explicit lifetime

```
One lifetime annotation by itself doesn't have much meaning because they are meant to describe relationships.

Why use?

```rust
// return a reference to the longer of the two &str params
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else { 
        y
    }
}
```
The get a compile error because the compiler can't tell if x will be longer or y will be longer and therefore doesn't know
what the lifetime of the returned reference will be. This is needed in order to determine if the reference we return will
always be valid.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
       ...
}
```
Now we are telling the compiler that the parameters passed in must have the same lifetime. This prevents code such as:

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longer string is: {}", result);
}
```
(Because we cannot use a reference to a string that will be out of scope. string2 gets dropped after the inner bracket is closed.)

Annotating struct definitions:
```rust
struct Excerpt<'a> {
    part: &'a str,
}
```
This means that the instance of `Excerpt` cannot outlive the reference it holds in its `part` field.

##### Input Lifetimes
lifetimes of functions or method parameters.

Rule: each parameter that is a reference gets its own lifetime parameter. 
(`fn foo<'a>(x: &'a i32)`,  `fn food<'a, 'b>(x: &'a i32, y: &'b i32)`)

##### Output lifetimes
lifetimes on return values

Rules: 
1. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
(`fn foo<'a>(x: &'a i32) -> &'a i32`)
2. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, 
   the lifetime of `self` is assigned to all output lifetime parameters.

##### Static lifetime
The `'static` lifetime means that the reference "can" live for the entire duration of the program. 
All string literals have the `'static` lifetime. `let s: &'static str = "This is a string literal";`

### Putting everything together
```rust
fn longest_with_notify<'a, T> {
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Notification: {}", ann);
    if x.len() > y.len() {
        x
     } else {
        y
     }
}
```


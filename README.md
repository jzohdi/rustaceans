
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
                                    // but i32 is Copy, so it???s okay to still
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

Rust implements generics in such a way that your code doesn???t run any slower using generic types
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

## Testing

Using the command `cargo test`

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        panic!("This test will fail");
    }
}
```
Important commands and marcros
```shell
cargo test
cargo test -- --help
cargo test -- --test-threads=1 // tells the compiler to not use prallelism
cargo test -- --show-output // shows println! statements main during testing
cargo test <function name> // tests a specific fn
cargo test <fn contaits> // ex: fn add_two fn add_three will both be run by cargo test add
cargo test -- --ignored // run the tested ignored with wth #[ignore] annotation
cargo test --test <file name> // rn the specified file for integration tests

assert!( bool )
assert_eq!( , )
assert_ne!( , )

```

Usually `#[derive(PartialEq, Debug)]` annotation is enough for your struct/enum.

Adding custom error message with assert:
```rust
let to_test = String::from("hello");
assert!(to_test.contains("world"),
        "Test did not contain world, value was {}",
         to_test)
```

Should panic
```rust
#[test]
#[should_panic]
fn test_panic() {
   some_function_that_will_panic();
}
```
More precisely, if we want to test that the specific panic happened that we expect.
```rust
#[test]
#[should_panic(expected = "Invalid argument")
....

```
In stead of assert, we can return Result for testing.

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 = 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```
Conventions

### Unit Tests

Put unit tests in the src directory in each file with the code that they're testing. The convention is to create a module named tests in each file
to contain the test functions and to annotate the module with `cfg(test)`.

Note that because of this structure you can also test private functions

### Integration Tests
In Rust, integration tests are entirely external to your library. We use a `tests/` at the top level next to `src/`.
To use shared code use a `/mod.rs` file. This will tell rust that this file is not for testing in the tests/ folder.

## Chapter 12
mingrep project

## Chapter 13
- Closures

### Iterators
- are lazy

All iterators implement a trait named `Iterator`, defined in the standard library.

```rust
pub trait Iterator {
    type Item; // defines an associated type with the trait

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default impl elided
}

```

Writing your own iterator

```rust
struct Count {
    count: u32
}

impl Count {
    fn new() -> Count {
        Count { count: 0 }
    }
}

impl Iterator for Count {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None 
        }
    }
}
```
Other tricks

```rust
let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
```

## More on Cargo and Crates

List of build and dev profile settings: https://doc.rust-lang.org/cargo/reference/profiles.html

Documentation comments using `\\\`

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
``` 

```shell
cargo doc --open
```
Will build the HTML for your current crate's documentation (as well as for the dependancies).
Other sections besides `#Examples` commonly used are `#Panics`, `#Errors` and `#Safety`. 
Also with the Examples, now running `cargo test` will test these doc tests.

## Chapter 14 
Buidling crates and workspaces see `workspaces/` for small example.

## Chapter 15
Smart Pointers, datastructures previously used like `String` and `Vec<T>` are examples of smart pointers, 
just did not address that at the time. 
Smart pointers implement the `Deref` and `Drop` traits, which allows a struct to behave like a reference and 
allows the struct to customize the code that is run when the smart pointer goes out of scope.

Smart pointer is a general design pattern frequently used in Rust.
A few examples include:
1. `Box<T>` for allocating values on the heap.
2. `Rc<T>` a reference counting type that enables multiple ownership.
3. `Ref<T>`/`RefMut<T>` accessed through `RefCell<T>` which enforces borrowing rules at runtime instead of at compile time.

### `Box<T>`

A type of smart pointer which allow allocating from the heap isntead of on the stack.

Common situations:
1. When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires exact size.
2. When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so.
3. When you want to own a value and you care only taht it's a type that implement a particular trait rather than being of a specific type.

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```
#### Box with recursive types
As mentioned before, Rust needs to know the size of objects at compile time. However, this isn't possible with recursive types. 

```rust
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
    ...
} // this will be an error
```
Instead do:
```rust
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    ...
}
```
The box provides indirection and heap allocation, and don't have any performance overhead other than the heap vs stack perf.

### Deref
Implementing the `Deref` trait allows you to customize the behavior of the dereference operator `*`.
We can see that `Box` can be dereferenced similar to other references.
```rust 
let x = 5;
let y = &x;
assert_eq!(5, *y);

vs

let x = 5;
let y = Box::new(x);
assert_eq!(5, *y);
```
Implementing this ourselves:

```rust
struct MxBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}
```
#### Deref coercion
Rust will automatically transform applicable type using a series of deref calls so that they match function params.
Example:
```rust
// we can do this
let m = MyBox::new(String::from("hello"));
hello(&m);

// instead of this
hello(&(*m)[...]);

//where
fn hello(statement: &str) {
...
}
```
Applicable cases:
1. From `&T` to `&U` when `T: Deref<Target=U` as with above
2. From `&mut T` to `&mut U` when `T: Deref<Target=U>`
3. From `&mut T` to `&U` when `T: Deref<Target=U>`

### Drop trait
Gets called when the struct is dropped (goes out of scope or manually). Note that you are not allowe to call the .drop fucntion.

```rust
struct MyStruct
    name: String,
}

impl Drop for MyStruct {
   pub fn drop(&mut self) {
       println("Dropping MyStruct with name: {}", self.name);
   }
}

use std::mem::drop;

fn main() {
    let a = MyStruct { name: String::from("Jake") };
    let b = MyStuct { name: String::from("Korvo") };

    drop(a); // a gets dropped, which calls a.drop();

    println!("name for b: {}", b.name);

 } // b gets dropped because out of scope and b.drop() is called.

```
### Rc<T>
Reference Counted Smart Pointer

In certain cases a single value might have multiple owners, like in a graph data structure where multiple edges refer to the same node. In this example a node should not be dropped unless it does not have an edges pointing to it.

`Rc<T>` is an abbreviation for reference counting, which keeps track of the number of refernce to a value which determines whether the value is still in use.

Note: This is only for use in single-threaded scenarios.

Example:
```rust
// bring Rc into scope and replace with Box from before.
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
Note that the above clone is not making a deep copy which could potentially be an expensive action, instead it is simply inrementing the count of the number of refernces to a.

We can get the number of references to a with `Rc::strong_count(&a);`
These reference will be immutable references (used for reads).

### `RefCell<T>`
Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data. Normally this is disallowed, but it is made possible using `unsafe` code inside the data structure.

Using ref cell will make the program compile but instead, will panic at runtime if the borrowing rules are broken.

Also only for single-threaded scenarios.

### Putting them together
We can combine these two to have multiple owners of data that can also mutate the data.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>, Rc<List>),
    Nil
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    
     println!("a after mut = {:?}", a);
     pirntln!("b after mut = {:?}", b);
     println!("c after mut = {:?}", c);
```
`Cell<T>` offers similar flexibility but internally uses `Mutex<T>` in order to provide safety.

### Reference Cycles
It is possible that a cyclic data structure could case memory leaks. This is because if
every item in the structure is refered to by something else, it will never be dropped.

Instead use `Weak<T>` which does not garauntee the reference is still held.
```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // returns an Option

    let root = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell:new(vec![Rc::clone(&leaf)),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // returns a smart pointer of type Weak<Node>

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 
```

## Chapter 16
Concurrency

Covered methods
1. creating multiple threads
2. Mesasge-passing concurrency
3. Shared-state concurrency
4. `Sync` and `Send` traits for user-defined types

## Chapter 18
Pattern Matching

### Literals
```rust
    let x = 1;
     
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

Matching named variables
```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {:?}", x, y)l;
```
Output
```shell
> Matched, y = 5
  at the end: x = Some(5), y = 10
```

### Matching Ranges of values with ..= and using |
```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything else"),
}

let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => prinln!("something else"),
}

let x = 'c';

match { 
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
     _ => println!("something else"),
}
```
Output
```shell
> one or two
  one through five
  early ASCII letter
```

### Destructuring

```rust
let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;

assert_eq!(0, a);
assert_eq!(7, b);

// or shorthand

let Point { x, y } = p;

assert_eq!(0, x);
assert_eq!(7, y);

match p {
    Point { x, y: 8 } => println!("this will not match"),
    Point { x: 0, y} => println!("x is 0, y = {}", y),
    Point { x, y } => println!("x is not 0 and y is not 8"),
}
```



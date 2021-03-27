
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

fn main() {

    println!("Hello world!");

    another_function(5);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    
    let z = five();

    println!("The value of z is: {}", z);

    let arr = [3;5]; // initializes an array on the stack of length 5 containing all values 3;

    for ele in arr.iter() {
        println!("The value is {}", ele);
    }

    for number in (1..4).rev() {
        println!("Countdown: {}", number); // .. get the range from 1 - 3 and .rev() reverses it
    }
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

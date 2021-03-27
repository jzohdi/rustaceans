use std::io;

fn main() {
    
    loop {

        let mut n = String::new();

        println!("Enter number n to compute the nth fibonacci");

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read input");

        if !is_num(&n) {
            continue;
        }
        let n: i32 = n.trim().parse().expect("Failed");

        println!("The {}th fibonacci number is {}", n, fib(n));
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
fn is_num(n: &str) -> bool {
    match n.trim().parse::<i32>() {
        Ok(_) => { println!("Is a number"); return true; },
        Err(e) => { println!("{} is not a number", n); println!("{}", e); return false },
    }
}

use std::io;

fn main() {

    let mut convert_to = String::new();
    let mut degrees = String::new();

    while convert_to != "F" && convert_to != "C" {

        println!("Would you like to convert to Fahrenheit or Celsius? [F/C]");

        io::stdin()
            .read_line(&mut convert_to)
            .expect("Failed to read line.");
        
        convert_to = convert_to.trim().to_string();

    }

    let from = if convert_to == "F" { "C" } else { "F" };
    let to = if convert_to == "F" { "Fahrenheit" } else { "Celsius" };

    loop {
        println!("Enter a number that will be converted to {}", if convert_to == "F" { "Fahrenheit" } else { "Celsius" });

        io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line.");

        match degrees.trim().parse::<f32>() {
            Ok(_) => { break; },
            Err(_) => { println!("Please enter a valid number"); continue; }
        };
    }

    let degrees: f32 = degrees.trim().parse().expect("Error");

    println!("{} {} in {} is {}", 
            degrees, from, to, convert(degrees, from));
}

fn convert(deg: f32, from: &str) -> f32 {
    if from == "F" {
       return (deg - 32.0) * 0.5556
    } else {
       return (deg * 1.8) + 32.0
    }

}

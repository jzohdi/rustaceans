fn main() {
    let days = ["first", "second", "third", 
                "fourth", "fifth", "sixth", 
                "seventh", "eight", "nineth", 
                "tenth", "eleventh", "twelfth"];

    let gifts = ["a partridge in a pear tree",
                 "turtle doves",
                 "French hens",
                 "calling birds",
                 "gold rings",
                 "geese a laying",
                 "swans a swimming",
                 "maids a milking",
                 "ladies dancing",
                 "lords a leaping",
                 "pipers piping",
                 "drummers drumming"];

    for (day_num, day) in days.iter().enumerate() {
        println!("\nOn the {} day of Christmas\nmy true love gave to me", day);

        for gift_num in (0..(day_num+1)).rev() {
            let f: String = if day_num > 0 && gift_num == 0 { String::from("and") } else { (gift_num + 1).to_string() };
            println!("{} {}",f, gifts[gift_num]);  
        }
    }
}


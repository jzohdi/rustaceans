use std::io;
use std::collections::HashMap;

pub fn employees() {
    println!("Enter your commands");
    println!("Valid commands:");
    println!("\tAdd <name> to <department> - adds employee to dept.");
    println!("\t<department>               - lists all employees in this dept.");
    println!("\nEnter q when done.");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");

        if command.trim() == "q" {
            break;
        }

        let words: Vec<&str> = command.trim().split(" ").collect();

        if words.len() == 1 && words[0] != "" {
            let dept = words[0];

            if let Some(names) = map.get_mut(dept) {
                names.sort();
                println!("{:?}", names);
            } else {
                println!("[]");
            }
        } else if is_valid_add(&words) {

            let name = get_name(&words);
            let dept = words[words.len() - 1].to_string();

            let mut lst = map.entry(dept).or_insert(vec![]);
            lst.push(name);
        } else {
            println!("Invalid command");
        }
    }
}

fn is_valid_add(words: &Vec<&str>) -> bool {
    if words.len() < 4 || words[0] != "Add" || words[words.len() - 2] != "to" {
        return false;
    }
    true
}

fn get_name(word_list: &Vec<&str>) -> String {
    let start = 1;
    let end = word_list.iter().position(|&s| s == "to").unwrap();

    String::from(word_list[start..end].join(" "))
}


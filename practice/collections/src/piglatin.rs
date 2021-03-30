extern crate unicode_segmentation;

use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub fn pig_latin() {
    let mut words = String::new();

    println!("Please enter a phrase to convert to pig latin.");

    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read line.");

    let words: Vec<String> = words.split(" ")
        .map(|s| pig_latinize_word(s.trim()))
        .collect();

    let words = words.connect(" ");
    println!("words: {:?}", words);
}

fn pig_latinize_word(word: &str) -> String {
    let mut final_str = String::new();
    let mut ending = String::new();

    for (index, g) in UnicodeSegmentation::graphemes(word, true).enumerate() {
        if index == 0 {
            let (first_char, end_chars) = latinize(g);
            final_str += &first_char;
            ending = end_chars;
        } else {
           final_str += &g; 
        }
    }
    format!("{}{}", &final_str, &ending)
}

fn latinize(g: &str) -> (&str, String) {
    if "aeiouAEIOU".contains(g) {
        return (g, String::from("-hay"));
    }
    ("", format!("-{}ay", g))
}

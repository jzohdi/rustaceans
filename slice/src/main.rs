fn main() {
    get_slice();
    slice_as_param();
}

// this shows that &str is a better param as it can be used with both String and &str types
fn slice_as_param() {
    let my_string = String::from("Hello world");
    
    let word = get_first_word(&my_string[..]);
    
    let my_literal = "hello world";

    let word = get_first_word(&my_literal[..]);

    // because literals are just slices anyways, we can pass as is
    let word = get_first_word(my_literal);
}

fn get_first_word(s: &str) -> &str {
    let as_bytes = s.as_bytes();

    for (i, &item) in as_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn get_slice() {
    let mut s = String::from("hello world.");

    let word = first_word(&s);

    // s.clear(); this would not compile since 
    // we cannot have a mutable borrow s.clear() 
    // and an immutable referrence at the same time (word)
    
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

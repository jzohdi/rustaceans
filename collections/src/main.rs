use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hash_maps();
}

fn hash_maps() {
    basics(); 
    collect();
    ownership();
    updating();
    counter();
}

fn counter() {
    let text = "hello world its a wonderful world world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn updating() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // 10 has been overwritten

    // only insert if the key does not exist in map
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); 
    println!("{:?}", scores);
}

fn ownership() {
    let name = String::from("favorite color");
    let value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(name, value);
    // after here name and value are invalid as the map is the owner of the data.
    // types like i32 who implement the `Copy` trait would still be valid
    map.insert(String::from("name"), String::from("jake"));

    let key = String::from("favorite color");
    println!("the map value for key '{}' is : {}", key, match map.get(&key) { Some(s) => s, None => "doesn't exist", });

    println!("All key: values");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

fn collect() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}
fn basics() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

}
fn strings() {
   let mut s = String::new();
   let s = "intial string".to_string(); // to_string method is available on any type that implements the Display trait
   let s = String::from("intial contents");
   updating_strings();
}

fn updating_strings() {
    // different ways of concating and updating strings
    let mut s = String::from("init");
    s.push_str("foo");

    let s = String::from(&s);
    let s2 = String::from(&s) + " " + &String::from(&s);

    println!("s : {}, s2: {}", s, s2); // can use s and s2 here

    let new_owned_string = s + &s2;
    
    println!("new_owned: {}, s2: {}", new_owned_string, s2); // cannot use s here bc new_owned took ownership

//    let borrowed: &str = "hello ";
    let borrowed2: &str = "world";

    let together = format!("{}{}", &new_owned_string, borrowed2);

    println!("together: {}", together);
}

fn vectors() {
    
    let with_out_macro: Vec<i32> = Vec::new();

    let with_macro = vec![1, 2 ,3]; // the macro infers the type

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(9);

    let third: &i32 = &v[2];
    println!("The third element of v is {}", third);

    match v.get(2) {
        Some(third) => println!("Third ele: {}", third),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i *= 2
    } // doubles each element in v

    println!("The third element of v is {}", &v[2]);
}

fn main() {
    println!("Hello, world!");
}
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
       // Overwriting a value?
    scores.insert(String::from("Blue"), 10);
 
//Only Inserting a Value If the Key Has No Value?
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Blue")).or_insert(50);
    }
}

fn main() {
   // let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

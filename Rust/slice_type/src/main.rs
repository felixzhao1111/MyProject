fn main() {
    println!("Hello, world!");
    let s = String::from("this is a program");
    let slice = first_word(&s);
    println!("{slice}");
    
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

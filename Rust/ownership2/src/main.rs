fn add_suffix(mut s: String) -> String {
    s.push_str(" world");
    s
}

fn main() {
    let s = String::from("hello");
    let x = add_suffix(s);
    // println!("{s}");
    println!("{x}");
}

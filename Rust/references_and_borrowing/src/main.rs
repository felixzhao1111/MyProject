/*
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");

    let mut s = String::from("hello");
    // rust只允许同时存在不可变引用，不允许同时存在可变引用和不可变引用
    // 但是由于NLL的存在，如果可变引用和不可变引用处在同一个作用域当中
    // 并且可变引用和不可变引用的定义和使用是完全分开的，那么也能编译通过
    // 下方是不可变引用r1、r2的定义和使用
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    // 下方是可变引用r3的定义和使用
    let r3 = &mut s; // 没问题
    println!("{}", r3);
    // 此位置之后 r3 不再使用

    // 下方是不可变引用r4的定义和使用
    let r4 = &s; // 没问题
    println!("{}", r4);
    // 此位置之后 r4 不再使用
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
*/

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

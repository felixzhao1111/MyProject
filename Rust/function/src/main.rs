fn main() {
    // Rust 代码中的函数和变量名使用 snake case 规范风格。
    // 在 snake case 中，所有字母都是小写并使用下划线分隔单词。
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {x}");
}
// Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。
// parameter 形参
// argument 实参
// 在函数签名中，必须声明每个参数的类型。
fn another_function(x:i32) {
    let a = "variable_a";
    println!("Another function.{a}{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 表达式返回值，语句不返回值，大括号，函数调用，宏调用都是表达式，其他的都是语句
fn five() -> i32 {
    5
}

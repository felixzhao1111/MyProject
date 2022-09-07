/*
// 引入std库的io
use std::io;
// 所有rust程序都从main函数开始
fn main() {
    // 创建一个String类型的变量
    let mut input = String::new();
    // 打印
    println!("hello");
    // 调用io的stdin的read_line获取用户输入
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("I say: {}", input);
        }
        _ => {}
    }
}
*/
fn main() {
    // 命名大于注释
    // 默认类型i32
    let i1 = 1;
    // 默认类型f64
    let f2 = 5.5;
    // 布尔类型
    let b3 = true;
    println!("i1 : {}", i1);
    println!("f2 : {}", f2);
    println!("b3 : {}", b3);
    // 单引号
    let c4 = 'c';
    println!("c4 : {}", c4);
    // 双引号
    let s5 = "ff";
    println!("s5 : {}", s5);
}

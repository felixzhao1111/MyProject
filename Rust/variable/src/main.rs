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
    // 加mut可以让变量可变，变量默认是不可变的
    // 常量必须在编译时就可以确定值是多少，可以在任何作用域中声明，通常用大写字母表示
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // addition，加法
    let sum = 5 + 10;

    // subtraction，减法
    let difference = 95.5 - 4.3;

    // multiplication，乘法
    let product = 4 * 30;

    // division，除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder，取余
    let remainder = 43 % 5;
    // 复合类型：元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 使用模式匹配获取元组中的元素
    let (x, y, z) = tup;
    // 使用索引获取元组中的元素
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // 数组和元组的区别是元素的类型必须相同
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // 5个3
    let a = [3; 5];
    // 使用索引来获取数组中的元素
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

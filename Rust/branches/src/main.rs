fn main() {
    // if是一个表达式，可以返回值
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    // 可以只声明变量名而不赋值
    let cond;
    // 声明时没有赋值，后面再进行赋值
    cond = true;
    // cond = false;  // 但是不能进行再次赋值，因为这是一个不可变变量
    let mut x;
    if cond {
        x = 1;
    } else {
        x = 2;
    }
    println!("{x}");
    x = 3;
    println!("{x}");
    // 空的花括号{}返回一个单元类型()，打印出来就是"()"
    let x = true;
    let y = if x {};
    println!("{y:?}");
}

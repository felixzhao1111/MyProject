fn main() {
    // 这里发生了拷贝，因为这是存放在栈上的数据
    let x = 5;
    let y = x;
    // 这里发生了移动，因为这是存放在堆上的数据
    let s1 = String::from("hello"); // -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    let s2 = s1; //  -- value moved here
                 // 打印s1报错了，因为s1的值已经被移动到了s2上
                 // println!("{}, world!", s1);
                 //                        ^^ value borrowed here after move
                 // 这里是一个拷贝堆上数据的例子
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

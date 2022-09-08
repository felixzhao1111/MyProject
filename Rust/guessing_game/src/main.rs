// 将 io输入/输出库引入当前作用域。io 库来自于标准库，也被称为 std
use std::io;
// Cargo.toml文件指定的版本是一个语义化版本
// Cargo.lock文件根据Cargo.toml生成，并且会锁定库的版本，如果要升级该版本号可以使用cargo update重新生成版本号，但是依然受限于语义化版本
// Rng 是一个 trait
use rand::Rng;
// Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。这是比较两个值时可能出现的三种结果。
use std::cmp::Ordering;
fn main() {
    // 打印字符串的宏
    println!("Guessing the number!");
    // 生成一个随机数，类型由后面的代码自动推断
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Your secret_number is {}", secret_number);
    // 无限循环
    loop {
        // 创建一个String类型的字符串，可变的
        let mut guess = String::new();
        // 调用 io 库中的函数 stdin
        io::stdin()
            // 调用 read_line 方法从标准输入句柄获取用户输入
            // 我们还将 &mut guess 作为参数传递给 read_line() 函数，让其将用户输入储存到这个字符串中
            .read_line(&mut guess)
            // read_line 会返回一个类型为 Result 的值。 Result 是一种枚举类型，通常也写作 enum。
            // Result 的实例拥有 expect 方法。
            // 如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。
            // 如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。
            .expect("Failed to read line!");
        // 使用guess:u32隐藏了原来的String类型的guess
        // trim()去掉字符串前后的空格
        // parse()将字符串转换成其他类型
        // 使用match而不是except，用了处理发生异常时的行为而不是直接抛出异常中断程序
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

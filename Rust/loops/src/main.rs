fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    loop_label();
    while_loop();
    for_loop();
    convert_temperature();
    fibonacci();
}

fn loop_label() {
    // 循环标签（loop label）
    // 用来将break或者continue与某层循环绑定，而不是作用在最内层的循环
    let mut count = 0;
    // 'counting_up标记了最外层的loop循环，在break后面跟上'counting_up即可结束最外层的循环
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    // while循环
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    // for循环，rev用来反转Range序列
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn input_number(s: &str) -> f64 {
    loop {
        println!("{s}");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to readline");
        let choice: f64 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入正确的选项！！!\n------------------");
                continue;
            }
        };
        return choice;
    }
}

fn convert_temperature() {
    // Convert temperatures between Fahrenheit and Celsius.
    loop {
        let c =
            input_number("1.摄氏度转华氏度\n2.华氏度转摄氏度\n0.退出\n请选择要功能所对应的编号：");
        if c == 1.0 {
            let celsius = input_number("请输入摄氏度：");
            let fahrenheit = celsius * 1.8 + 32.0; // 32°F+ 摄氏度 × 1.8
            println!("转换后的华氏度为：{fahrenheit:.2}°F");
        } else if c == 2.0 {
            let fahrenheit = input_number("请输入华氏度：");
            let celsius = (fahrenheit - 32.0) / 1.8; //  (华氏度 - 32°F) ÷ 1.8
            println!("转换后的摄氏度为：{celsius:.2}°C");
        } else if c == 0.0 {
            println!("退出！！!\n------------------");
            return;
        } else {
            println!("请输入正确的选项！！!\n------------------");
            continue;
        }
    }
}

fn input_number2(s: &str) -> u32 {
    loop {
        println!("{s}");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to readline");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入正确的选项！！!\n------------------");
                continue;
            }
        };
        return choice;
    }
}

fn fibonacci() {
    loop {
        // Generate the nth Fibonacci number.
        let n = input_number2("请输入需要生成几阶的斐波那契数列：");
        if n == 1 {
            println!("1");
        } else if n == 2 {
            println!("1 1")
        } else if n == 0 {
            return;
        } else {
            print!("1 1 ");
            let mut a = 1;
            let mut b = 1;
            let mut c;
            for i in (3..=n) {
                c = a + b;
                print!("{c} ");
                a = b;
                b = c;
            }
            println!(" ")
        }
    }
}

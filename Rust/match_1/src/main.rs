#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Location {
    Point(i32),
    Range(i32, i32),
}

fn print_range_max(loc: &Location) {
    // print the second field of Range, if loc is a Range
    if let Location::Range(x, y) = loc {
        println!("{y}");
    }
}


fn get_start(loc: &Location) -> i32 { 
    // return the first field of Range or the only field of Point  
    match loc {
        Location::Point(x) => x.clone(),
        Location::Range(y, z) => y.clone(),
    }
}

fn main() {
    // match和枚举的结合是很常见的模式
    let x = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{x}");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{six:?}");
    print_range_max(&Location::Range(94, 77));
    let x = get_start(&Location::Point(951));
    let y = get_start(&Location::Range(94, 77));
    println!("{x} {y}");
}

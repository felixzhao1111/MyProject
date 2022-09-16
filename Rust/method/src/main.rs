#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let rect4 = Rectangle::square(99);
    println!("rect4 {:?}", rect4);
    println!("rect4 {:?}", rect4.area());
}

impl Rectangle {
    // 在impl块中且有self参数的叫方法，实例通过.调用
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }
    // 在impl块中且没有self参数的方法叫做关联函数，通过对应类型的::调用
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// 一个结构体可以拥有多个impl块
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

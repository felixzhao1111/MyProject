// 结构体的定义
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体的定义
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体（没有值）
struct AlwaysEqual;
fn main() {
    // 结构体的实例化
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 可变必须是针对整个结构体的，而不是某个字段
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 修改某个结构体实例的字段的值
    user2.email = String::from("anotheremail@example.com");
    // 不使用结构体更新语法时，创建新的结构体的方法如下
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // 使用了结构体更新语法时，创建新的结构体的方法如下
    // ..user1会将其余字段的值从user1移动或拷贝到user2
    // 这取决于这个字段是否实现了Copy trait
    let user4 = User {
        username:String::from("ddd"),   // 如果注释掉这一行会导致user1无法使用
        email: String::from("another@example.com"),
        ..user1
    };
    // 需要注意的是，如果有某个字段发生了移动，旧的结构体将会无法使用

    // 即便Color和Point元组结构体有着相同的元素类型，但他们依然是不同的结构体
    // 假如一个参数需要一个Color，那么传入Ponit则会报错
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // 元组结构体的使用方式类似于元组，可以使用.后跟索引来访问单独的值
    
    // 类单元结构体的实例
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User { // 结构体可以作为函数的返回值
    // 如果结构体的字段名和参数名相同，则可以简写
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 不可修改静态变量
static LANGUAGE: &str = "Rust";
// 可修改静态变量
static mut COUNTER: i32 = 0;
// PS: 静态变量 偏向与长期存在的，会被修改的全局变量值

// 不可修改的常量
const THRESHOLD: i32 = 10;
// PS: 常量 偏向与长期存在的不会被修改的全局值

fn main() {
    // 可修改变量
    //let a: i32 = 1;
    // 不可修改变量
    // let mut b: i32 = 2;
    println!("This is a program written in {}.", LANGUAGE);

    let mut count = 0;
    while count < THRESHOLD {
        count += 1;
        println!("Count is: {}", count);
    }

     // 标记一个 unsafe 代码快
    // 请注意，不安全代码块是存在风险的实践，应仅在必要时使用。
    unsafe {
        COUNTER += 1;
        println!("Counter is now: {}", COUNTER);
    }
}

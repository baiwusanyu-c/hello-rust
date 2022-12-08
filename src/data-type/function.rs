// * 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
// * 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
// * 每个函数参数都需要标注类型
// 函数的返回值就是函数体最后一条表达式的返回值
fn plus_five(x:i32) -> i32 {
    x + 5
}

fn main() {
    let x = plus_five(5);
    println!("The value of x is: {}", x);
}
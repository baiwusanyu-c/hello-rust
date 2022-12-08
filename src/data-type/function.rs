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

// 永不发散的函数
// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，
// 特别的，这种语法往往用做会导致程序崩溃的函数：
// fn dead_end() -> ! {
//   panic!("你已经到了穷途末路，崩溃吧！");
// }
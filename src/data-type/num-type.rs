// 使用包依赖的某个方法
use num::complex::Complex;
fn base_var_num(){
    // 整数变量分为8 ~ 128 位，的有符号和无符号类型 i8 ~ i128、u8 ~ u128，
    // 以及是框架而定的, isize	usize

    // 浮点类型为 32 和 64 位 f32、f64，（注意浮点数陷阱）

    // NaN， 所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较
    let x = (-42.0_f32).sqrt();
    assert_eq!(x, x);

    // 序列 1..5，生成从 1 到 4 的连续数字，不包含 5 ；1..=5，
    // 生成从 1 到 5 的连续数字，包含 5，它的用途很简单，常常用于循环中：
    // 序列也可用于生成字母

    // 使用库进行有理数计算
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
 
    println!("{} + {}i", result.re, result.im)

}
// 使用包依赖的某个方法
use num::complex::Complex;
fn main() {
    // let a = Complex { re: 2.1, im: -1.2 };
    // let b = Complex::new(11.1, 22.2);
    // let result = a + b;

    // println!("{} + {}i", result.re, result.im)

    // Vec是动态数组
    let mut stack = Vec::new();
    
    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

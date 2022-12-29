fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// 報錯 该函数的返回值到底引用 x 还是 y ，因为编译器需要知道这些，
// 来确保函数调用后的引用生命周期分析。
// &i32        // 一个引用
// &'a i32     // 具有显式生命周期的引用
// &'a mut i32 // 具有显式生命周期的可变引用
fn longest(x: &'a str, y: &'a str) -> &'a str {
    // longest 函数并不知道 x 和 y 具体会活多久，
    // 只要知道它们的作用域至少能持续 'a 这么长就行。
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：
// * 函数参数的生命周期
// * 函数体中某个新建引用的生命周期

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// &'static 对于生命周期有着非常强的要求：一个引用必须要活得跟剩下的程序一样久，才能被标注为 &'static。
// 对于字符串字面量来说，它直接被打包到二进制文件中，永远不会被 drop，因此它能跟程序活得一样久，自然它的生命周期是 'static。
// 但是，&'static 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，还是要遵循相应的作用域规则 :
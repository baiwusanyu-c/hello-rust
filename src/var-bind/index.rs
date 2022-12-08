
struct Struct {
    e: i32
}
fn var_bind() {
    let a = "非常扯，let定义变量默认定义后不可改变";
    println!("{}", a);

    let mut b = "真傻逼啊";
    println!("{}", b);
    b = "如果想让变量能够修改，必须使用 mut 修饰,let mut a = 'foo'";
    println!("{}", b);

    let _c = "使用下划线开头忽略未使用的变量";

    // 变量解构
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    // {:?} 是打印模板诶
    println!("变量解构 ：a = {:?}, b = {:?}", a, b);
    b = true;
    // 断言 ？？
    assert_eq!(a, b);

     // 解构赋值
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    const CONST_VAR: &str = "常量和 js 一样使用 const 关键字";
    println!("{}", CONST_VAR);

    // 变量覆盖遮隐
    let re_declare = "rust 变量可以重复定义同名变量";
    // 字符串拼接相加必须在后面那个变量前加个&
    let re_declare = re_declare.to_string() + &",并且有块级作用域";
    {
        let re_declare ="块内，可以访问上一级作用域变量使用，\n
        但是重复声明不会覆盖上级作用域的同名变量,\n
        和 mut 相比，重复定义来修改变量会重新分配内存，化不戳\n";
        println!("{}", re_declare);
    }
    println!("{}", re_declare);

}

// 字符类型
fn charType(){
    // 字符类型使用单引号来定义 '',且只有一个字母
    const charTest:char = 'c';
    println!("{}", charTest);
}

// 布尔类型
fn boolType(){
    // 字符类型使用单引号来定义 '',且只有一个字母
    const falseVar:bool = false;
    const trueVar:bool = true;
    println!("{}", charTest);
}

// 单元类型
fn unitType(){
   // 单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 ()
   // 没错， main 函数就返回这个单元类型 ()，你不能说 main 函数无返回值，
   // 因为没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数。
}
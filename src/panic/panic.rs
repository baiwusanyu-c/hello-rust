use std::fs::File;
fn demoPanic(){
    // panic 宏，当调用执行该宏时，程序会打印出一个错误信息，
    // 展开报错点往前的函数调用堆栈，最后退出程序。
    panic!("crash and burn");

    // result
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
    // unwrap 和 expect。它们的作用就是，如果返回成功，
    // 就将 Ok(T) 中的值取出来，如果失败，就直接 panic，真的勇士绝不多 BB，直接崩溃。
    let f = File::open("hello.txt").unwrap();

    // ? 宏 作用类似于match
    // 如果结果是 Ok(T)，则把 T 赋值给 f，如果结果是 Err(E)，
    // 则返回该错误，所以 ? 特别适合用来传播错误。
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
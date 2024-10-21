use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    // rand::thread_rng() 获取随机数生成器
    // gen_range 生成随机数， 1 <= x <= 101
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        // 创建可变的字符串变量，用于存储用户输入
        let mut guess = String::new();
        // 从标准库 io 中调用 stdin 函数，得到 stdin 实例
        io::stdin()
            // stdin 实例的 read_line 方法，该方法传入一个可变指针，
            // 接受用户输入后根据传入的可变指针参数，修改变量，将用户输入赋值给变量
            // 返回 Result 类型，使用 expect 方法处理 Result 的 Err 变体
            .read_line(&mut guess)
            // Result
            // -- OK(usize)
            // -- Err 使用 expect 处理，程序直接退出
            .expect("Failed to read line");
        // 调用 trim() 去除空格
        // 调用 parse() 去自动解析为 u32 类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        // guess.cmp 进行大小匹配得到 Ordering 枚举
        match guess.cmp(&secret_number) {
            // Ordering 来自标准库的变体，用于比较大小
            Ordering ::Less => println!("Too small!"),
            Ordering ::Greater => println!("Too big!"),
            Ordering ::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}

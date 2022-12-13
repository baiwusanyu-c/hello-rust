// @（读作 at）运算符允许为一个字段绑定另外一个变量。
// 下面例子中，我们希望测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，
// 同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它。
fn AtBindDemo() {
    struct Point {
        x: i32,
        y: i32,
    }
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello { id: id_variable @ 3..=7 } => {
                println!("Found an id in range: {}", id_variable)
            },
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            },
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            },
        }
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};

    // rust 1.53 特性
    // 匹配 1，这里将 (1 | 2) 通过 @ 绑定到 num 上，
    // num 与 1 匹配，此时num可取值为 1 或 2 匹配成功
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
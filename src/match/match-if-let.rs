// 模式匹配 match(相当于 switch), _ 类似于 switch 中的 default。
// * match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
// * match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
// * X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
enum Direction {
    East,
    West,
    North,
    South,
}

fn matchDemo(){
    let dire = Direction::South;
    // match 模式也是一个表达式，可以直接将匹配结果赋值给变量
    let res = match dire {
        1..=5 => println!("..= 语法允许你匹配一个闭区间序列内的值"),
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}

// 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
// 匹配 v 是否为 3 (some 是标准库的 enum Option<T>)
fn ifLetDemo(v: i32){
    if let Some(3) = v {
        println!("three");
    }
}

// matches! 宏进行匹配,可以与枚举进行匹配
// enum MyEnum {
//     Foo,
//     Bar
// }
// let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
// v.iter().filter(|x| matches!(x, MyEnum::Foo)); 用宏匹配出数组中的 MyEnum::Foo
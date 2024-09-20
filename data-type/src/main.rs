fn main() {

    //  字节字面量写作 b'x'，x 可以是任何 ASCII 字符或转义字符。
    // 例如，A 的 ASCII 码为 65，字面量 b'A' 和 65u8 完全等效。
    #[allow(non_snake_case)] // 去除警告
    let A = b'A';
    println!("A is {}", A); // A is 65


    // 整型之间不能进行隐式的转化，必须进行显式的转换。
    // 整型之间的转换
    // 使用 as 进行转换。超出取值范围时，转换会导致数据丢失。
    let a: i32 = 300;
    let b: u8 = a as u8;
    let c: i64 = a as i64;

    println!("i32: {}", a); // 300
    println!("u8: {}", b);  // 44 转换为 u8 类型，只有值在 u8 的范围内才安全
    println!("i64: {}", c); // 300

    // try_from() 和 try_into() 方法进行尝试，转换失败时，返回错误。
    // try_from() 定义在 std::convert::TryFrom trait 中，尝试将一个类型转化为另一个类型。
    // try_into() 定义在 std::convert::TryInto trait 中，尝试将一个类型转化为另一个类型。

    // let d: i32 = 300;
    // try_from
    // let f = u32::try_from(d).expect("1 数值超出了 u8 的范围");
    // println!("f: {}", f); // f: 300
    // let e = u8::try_from(d).expect("2 数值超出了 u8 的范围");
    // println!("e: {}", e); // 数值超出了 u8 的范围: TryFromIntError(())

    // try_into
    // let g: u32 = d.try_into().expect("3 数值超出了 u8 的范围");
    // println!("g: {}", g); // g: 300
    // let h = u8::try_from(d).expect("4 数值超出了 u8 的范围");
    // println!("h: {}", h); // 数值超出了 u8 的范围: TryFromIntError(())

    assert_eq!(2_u16.pow(4), 16);            // 求幂
    assert_eq!((-4_i32).abs(), 4);           // 求绝对值
    assert_eq!(0b101101_u8.count_ones(), 4); // 求二进制1的个数”


}

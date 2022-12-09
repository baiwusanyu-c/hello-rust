
//  切片 &str
fn StringSliceDemo(){
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn StringDemo(){
    // &str 和 String::form 不是同一个东西哦
    let strSliceTest = "hello &str"; // 字符串字面量就是切片
    let strTest = String::form("hello String::form");

    // ********************************** 追加 ****************************************    
    // push 追加字符
    strTest.push('F');
    println!("插入字符 push() -> {}", strTest);
    // push_str 追加字符串
    strTest.push_str("uck");
    println!("插入字符串 push_str() -> {}", strTest);

    // push 追加字符
    strTest.insert(5, '$');
    println!("插入字符 insert() -> {}", strTest);
    // push_str 追加字符串
    strTest.insert_str(6, "@#$");
    println!("插入字符串 insert_str() -> {}", strTest);

  // ********************************** 替换 ****************************************    

    let string_replace = String::from("I like rust. Learning rust is my favorite!");

    // 该方法可适用于 String 和 &str 类型。 该方法会替换所有匹配到的字符串。
    // 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let new_string_replace = string_replace.replace("rust", "RUST");

    // 该方法可适用于 String 和 &str 类型。replacen() 方法接收三个参数，前两个参数与 replace() 方法一样，
    // 第三个参数则表示替换的个数。该方法是返回一个新的字符串，而不是操作原来的字符串。
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);

    // 该方法仅适用于 String 类型。replace_range 接收两个参数，第一个参数是要替换字符串的范围（Range），
    // 第二个参数是新的字符串。该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");

    // ********************************** 删除 ****************************************    
    // 四个方法全部操作原字符串
    // pop —— 删除并返回字符串的最后一个字符
    // remove(pos) —— 删除并返回字符串中指定位置的字符
    // truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    // clear —— 清空字符串 

    // ********************************** 连接 ****************************************    
    // 使用 + 或者 += 连接字符串, + 和 += 都是返回一个新的字符串。所以变量声明可以不需要 mut 关键字修饰。
    // 要求右边的参数必须为字符串的切片引用（Slice）类型。其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法，
    // 这里 add() 方法的第二个参数是一个引用的类型。因此我们在使用 +， 必须传递切片引用类型。不能直接传递 String 类型。
    // let str = s1 + &s2， add方法第一个参数可以是 string 类型，而第二个开始后，需要用 引用类型 &s2
    // 并且，s1 的所有权会被转移到 add方法中，add 方法运行完， s1 的存储堆内存施法（栈内存中s1 的指针还在）
    // 此时，add 方法运行完（+ 完之后）,再访问 s1 会报错
}
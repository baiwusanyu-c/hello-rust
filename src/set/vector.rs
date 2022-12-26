fn demoVector(){
    // 动态数组，数组元素类型为 i32 类型
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);
    // get方法访问数组越界时，会返回None，
    // 使用相对安全
    println!("第一个元素是 {}", v.get(0));
    // 动态数组遍历
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 10
    }
}
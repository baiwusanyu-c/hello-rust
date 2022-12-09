
fn IfElseDemo(){
    let condition = true;
    // 条件语句
    if condition == true {
        println!("The value of number is: {}", 'true');
    } else {
        println!("The value of number is: {}", 'false');
    }
    // 条件赋值，注意每个分支上的返回值类型必须一直
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}


fn forDemo(){
    // 从 1 到 5 输出序列
    // 使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合
    for i in 1..=5 {
        println!("{}", i);
    }

      // 如果想在循环中，修改该元素，可以使用 mut 关键字：
    let collection = [1, 2, 3, 4, 5];
    for item in &mut collection {
        // ...
    }
    // for item in collection === for item in IntoIterator::into_iter(collection)	转移所有权
    // for item in &collection === for item in collection.iter()	不可变借用
    // for item in &mut collection === for item in collection.iter_mut()	可变借用

     // 获取元素的索引, `.iter()` 方法把 `a` 数组变成一个迭代器
     for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }


    // 第一种
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
    let item = collection[i];
    // ...
    }

    // 第二种
    for item in collection {
        if item == 2 {
            continue;
        }
        if i == 3 {
            break;
        }
    }
    // 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— 
    // Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
    // 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，
    // collection 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险（这里是因为所有权吗？是的话可能要强调一下）
}
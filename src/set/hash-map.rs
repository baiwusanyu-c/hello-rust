use std::collections::HashMap;
    fn demoHashMap() {
    // 创建一个HashMap，用于存储宝石种类和对应的数量
    let mut my_gems = HashMap::new();

    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    // 数组转map
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    // into_iter 方法将列表转为迭代器，接着通过 collect 进行收集
    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    // HashMap 的所有权规则与其它 Rust 类型没有区别：
    //若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
    //若没实现 Copy 特征，所有权将被转移给 HashMap 中

    // 遍历读取
    for (key, value) in &teams_map {
        println!("{}: {}", key, value);
    }
     // 查询Yellow对应的值，若不存在则插入新值
     let v = my_gems.entry("Yellow").or_insert(5);
}
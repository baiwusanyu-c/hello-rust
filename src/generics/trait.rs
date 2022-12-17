// 特征
fn demoTrait(){
    // 特征定义了一个可以被共享的行为，只要实现了特征，你就能使用该行为。
    // 定义一个特征，该特征共享的一个方法summarize
    pub trait Summary {
        fn summarize_author(&self) -> String;
        // 特征内方法可以调用 这里调用了 summarize_author
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    pub struct Post {
        pub title: String, // 标题
        pub author: String, // 作者
        pub content: String, // 内容
    }
    
    // 实现结构体类 Post，并包含特殊 Summary 的具体实现
    // 即为 Post 类型实现 Summary 特征
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    // 使用特征作为函数参数
    // 你可以使用任何实现了 Summary 特征的类型作为该函数的参数，
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // 多重约束 pub fn notify(item: &(impl Summary + Display)) {}
    // 同时实现了这两个特征的参数才能被传入

    // 通过 derive 派生特征
    // 这种是一种特征派生语法，被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能。
    // 例如 Debug 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，
    // 就可以使用 println!("{:?}", s) 的形式打印该结构体的对象。

    // 在脚本顶部 引入特征，use std::convert::TryInto;
    // 某些特征，rust 会自动加入作用域，可以直接使用特征方法 

}
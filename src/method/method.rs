fn demoMethod(){
    // 基于实现类来创建对象定义方法
    pub struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    pub impl Circle {
        // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
        // 这种方法往往用于初始化当前结构体的实例
        // 相当于构造函数
        pub fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }

        // Circle的方法，&self表示借用当前的Circle结构体
        // 一般来说，方法跟字段同名，往往适用于实现 getter 访问器， ⚠️ 这会使得同名字段变为私有
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    // 使用时
    let cirlcle = Circle {
        x: 1.2,
        y: 1.2,
        radius: 1.2,
    }
    // 或
    /**
     * Circle::new{
        x: 1.2,
        y: 1.2,
        radius: 1.2,
    }
     */

    cirlcle.area()

    // 枚举实现类
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

}
// 特征对象

fn demoTraitObject(){
    // 定义一个特征
    pub trait Draw {
        fn draw(&self);
    }
    // 特征对象指向实现了 Draw 特征的类型的实例
    // 这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。
    // 可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。

    // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
    n draw1(x: Box<dyn Draw>) {
       // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
       x.draw();
    }

    n draw2(x: &dyn Draw) {
       x.draw();
    }
    // self指代的就是当前的实例对象
    // Self 用来指代当前调用者的具体类型
}
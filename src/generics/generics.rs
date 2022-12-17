fn demoGenerics(){
    // T 为泛型, std::cmp::PartialOrd 是泛型约束
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
        let mut largest = list[0];
    
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }

    // 结构体中的泛型
    struct Point<T> {
        x: T,
        y: T,
    }
    // 枚举中的泛型
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 方法中的泛型
    struct Point<T> {
        x: T,
        y: T,
    }
    
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    // const 值的泛型，下面是通过const 给数组长度值定义了一个泛型
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    
}
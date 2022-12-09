//如 object、 record 
// 好像 interface 啊
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 结构体支持解构。。。和属性同名省略这种写法
// 这不就是抄 js/ts
fn structDemo(){
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

  // 根据已有结构体创建新的结构体
  // user1 的 username 的所有权发生转移，导致
  // user1 无法使用，而user1 的 active， sign_in_count 会发生 copy
  let user2 = User {
    email: String::from("another@example.com"),
    ..user1 
    };

    // 结构体必须要有名称，但是结构体的字段可以没有名称，
    // 这种结构体长得很像元组，因此被称为元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    // 单元结构体
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，
    // 因此将它声明为单元结构体，然后再为它实现某个特征
    impl SomeTrait for AlwaysEqual {}
}

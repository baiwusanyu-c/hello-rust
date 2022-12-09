// rust 第一种是速度很快但是长度固定的 array，
// 第二种是可动态增长的但是有性能损耗的 Vector
//  !! 在 rust 中，数组的中元素的类型必须全部一样
fn arrayDemo() {
    // 固定数组，已知长度大小，他是存储在栈上面的，而 Vector 动态数组存储在堆上
    // 类型为 i32，长度 5
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 数组上存储复杂类型 应该调用std::array::from_fn
    let array: [String; 8] = core::array::from_fn(|i| String::from("rust is good!"));
    println!("{:#?}", array);

     // 编译器自动推导出one的类型
  let one             = [1, 2, 3];
  // 显式类型标注
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
  let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

  // 借用arrays的元素用作循环中
  for a in &arrays {
    print!("{:?}: ", a);
    // 将a变成一个迭代器，用于循环
    // 你也可以直接用for n in a {}来进行循环
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
  }
    
}

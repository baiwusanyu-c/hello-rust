### 整型溢出
假设有一个 u8 ，它可以存放从 0 到 255 的值。那么当你将其修改为范围之外的值，比如 256，
则会发生整型溢出。关于这一行为 Rust 有一些有趣的规则：当在 debug 模式编译时，
Rust 会检查整型溢出，若存在这些问题，则使程序在编译时 panic(崩溃,Rust 使用这个术语来表明程序因错误而退出)。

在当使用 --release 参数进行 release 模式构建时，Rust 不检测溢出。相反，
当检测到整型溢出时，Rust 会按照补码循环溢出（two’s complement wrapping）的规则处理。
简而言之，大于该类型最大值的数值会被补码转换成该类型能够支持的对应数字的最小值。
比如在 u8 的情况下，256 变成 0，257 变成 1，依此类推。程序不会 panic，
但是该变量的值可能不是你期望的值。依赖这种默认行为的代码都应该被认为是错误的代码。

### 位运算

| 运算符    | 说明                         |
|--------|----------------------------|
| & 位与	  | 相同位置均为1时则为1，否则为0           |
| ㅣ 位或     | 相同位置只要有1时则为1，否则为0 |
| ^ 异或	  | 相同位置不相同则为1，相同则为0           |
| ! 位非	  | 把位中的0和1相互取反，即0置为1，1置为0     |
| << 左移	 | 所有位向左移动指定位数，右位补0           |
| >> 右移	 | 所有位向右移动指定位数，带符号移动（正数补0，负数补1 |

### 所有权

- 垃圾回收机制(GC)，在程序运行时不断寻找不再使用的内存，典型代表：Java、Go
- 手动管理内存的分配和释放, 在程序中，通过函数调用的方式来申请和释放内存，典型代表：C++
- 通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查

rust中
- 1.Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
- 2.一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
- 3.当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

```
// String::from 创建了一个所有权给到s1，通过赋值绑定，把所有权给到了s2
let s1 = String::from("hello");
let s2 = s1;
// : &strs 创建一个静态字符串，没有创建所有权，
// 这里只是把存储的的引用指向s3，又通过赋值绑定，使得s4 也引用
let s3: &str = "hello, world";
let s4 = s3;
```
Rust 这样解决问题：当 s1 赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。

### 浅拷贝与深拷贝
rust 不会主动深拷贝，有一个.clone方法能够完整复制

#### rust 的 copy（浅拷贝）
浅拷贝只发生在栈上
Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。

那么什么类型是可 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则： 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：

* 所有整数类型，比如 u32。  
* 布尔类型，bool，它的值是 true 和 false。  
* 所有浮点数类型，比如 f64。  
* 字符类型，char。  
* 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。  
* 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的  

### 引用与借用
```
fn main() {
    let x = 5;
    // & 创建堆 x 的指针引用
    let y = &x;

    assert_eq!(5, x);
    // *y 解引用拿到其引用值，即 x 的 5
    assert_eq!(5, *y);
    // 报错，不可整数和引用比较
    assert_eq!(5, y);
}
```
一般来讲，通过 & 创建的引用，是不可修改的，但可以使用 &mut 来创建一个可变引用，
！！！同一作用域，特定数据只能有一个可变引用
！！！可变引用与不可变引用不能同时存在
```
报错 
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```
#### 悬垂引用
```
fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
```

swc 是基於 rust 的生態，因此不管是swc本身還是其基於swc的插件，都是基於rust編寫，
最終編譯成wasm文件，在node中執行，實現能夠去支持前端工具鏈能力
rust -> swc 
                  -> vite-plugin
rust -> swc-plugin

wasm文件是基於WebAssembly的可執行文件， WebAssembly是下一代Web客户端开发技术，目前已经是W3C的标准，94%的浏览器已经支持了WebAssembly标准。WebAssembly技术通过加载非Javascript语言（如C/C++、Rust、Go等）编写的源码经过交叉编译后生成的二进制文件，然后通过Javascript API调用相关函数完成计算，由于这些计算模块采用编译后的接近机器码的方式运行，因此WebAssembly技术可以让代码运行效率接近本机的运行效率，这大大提高了Web客户端程序的运行效率。当然，不建议使用WebAssembly操作浏览器DOM，否则运行效率可能会更低。

例如你的rust脚本包含一個add函數，編譯成wasm文件后，可以通過js進行加載編譯，最終可以通過js直接調用這個add函數
```javascript
 function loadWebAssembly(fileName) {
            return fetch(fileName)
                .then(response => response.arrayBuffer())
                .then(buffer => WebAssembly.compile(buffer)) // 编译
                .then(module => { return WebAssembly.instantiate(module) }); //创建WebAssembly实例
        };

        //调用加载WebAssembly函数，注意wasm文件必须要本html文件在服务器同一目录，否则可能会出现404错误
        loadWebAssembly('hellowasm.wasm').then(instance => {
            console.time("WebAssembly")
            console.log(instance.exports.add(45,1))
          
        });
```
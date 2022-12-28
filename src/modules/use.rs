

// as 别名引用
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

// 引用再导出
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

// 使用 {} 简化引入方式
use std::collections::{HashMap,BTreeMap,HashSet};
use std::{cmp::Ordering, io};
// use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy
use std::io::{self, Write};
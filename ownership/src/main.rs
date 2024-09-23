fn main() {
    let mut s = "hello".to_string();
    let t = s;
    // println!("{}", s);
    s = "world".to_string();
    println!("{}", s);
    println!("{}", t);
}

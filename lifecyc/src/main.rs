fn main() {
    // 引用
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    //  The length of 'Hello' is 5.
}

fn calculate_length(s: &String) -> usize {
    s.len() // (*s).len()
}

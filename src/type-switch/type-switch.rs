use std::convert::TryInto;
fn demoTypeSwitch(){
    let a: u8 = 10;
    let b: u16 = 1500;
 
    let b_: u8 = b.try_into().unwrap();
 
    if a < b_ {
      println!("Ten is less than one hundred.");
    }
}
// 解构赋值
fn toDeconstructDemo(){
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    fn structDe() {
        let p = Point { x: 0, y: 7 };
    
        let Point { x, y:a } = p;
        assert_eq!(0, x);
        assert_eq!(7, a);
    }

    // 解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    fn enumDe() {
        let msg = Message::ChangeColor(0, 160, 255);
    
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x,
                    y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                )
            }
        }
    }
    
    // 解构元组
    let ((feet, inches)) = ((3, 10));

    // 解构数组
    let arr: [u16; 2] = [114, 514];
    let [xArr, yArr] = arr;

    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
}
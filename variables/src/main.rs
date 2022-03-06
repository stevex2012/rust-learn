const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Hello, world!");

    let guess: u32 = "42".parse().expect("not a number");

    println!("{}",guess);
    
    let mut x = 5;
    println!("value {}", x);

    x = 6;
    println!("value {}", x);

    let y = 8;
    // y = y +1; 
    let y = y+1;// 使用隐藏
    let y = y*2;
    println!("value {}", y);

    // 类型不一样
    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}", spaces);

    // 数据类型
    let str = 2.0; // 64
    let str1: f32 = 3.0;

    // 数据类型 +-*/%

    let t = true;

    let z = 'z';
}

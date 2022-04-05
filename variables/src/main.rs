// const MAX_POINTS: u32 = 100_000;

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
    // let str = 2.0; // 64
    // let str1: f32 = 3.0;

    // 数据类型 +-*/%

    // let t = true;

    // let z = 'z';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.5, 1);

    // let (x,y,z) = tup;

    // println!("{},{},{}", x,y,z)
    println!("{},{},{}", tup.0, tup.1,tup.1);


    // 数组
    let a = [1,2,3,4,5];
    let b = [3;5];
    println!("{},{}", a[0], b[2]);

    // function  fn x_x(){} parameters arguments
    a_fn(5,6);

    let xxx = five(55);

    println!("the value is {}", xxx);

    if_el_fn();

    loop_fn();

    string_fn();
}

fn string_fn(){
    let mut s = String::from("hello");
    s.push_str(", world");
    // let s2 = s; //move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
    println!("{}",s)
}

fn a_fn(x: i32, y:i32){
    println!("a_fn {}", x);
    println!("a_fn {}", y)
}


fn five(x:i32) -> i32{
    return x+5;
}
// if else
fn if_el_fn(){
    let num = 3;
    if  num < 5 {
        println!("big")
    } else {
        println!("samll")

    }
}
// 循环 for 在rust里面用的最多
fn loop_fn(){
    let a = [10,20,30,40,50];
    for element in a.iter(){
        println!("the value is: {}", element);
    }

    for number in (1..4).rev(){
        println!("{}!", number)
    }

    println!("liftoff!")

}
// 所有权


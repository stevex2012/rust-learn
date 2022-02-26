use std::io; //prelude 引用库
use rand:Rng; // trait

fn main() {
    println!("猜数");

    println!("猜测一个数");

    // let mut foo = 1;
    // let bar = foo;// immutabel; 
    // foo = 23;

    let secret_number = rand::thread_rng().gen_rang(1,101)

    println!("得到一个随机数 {}", secret_number)

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");
    //

    print!("你的猜测是：{}", guess)
}

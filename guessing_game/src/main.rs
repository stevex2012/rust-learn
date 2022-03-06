use std::io; //prelude 引用库
use rand::Rng; // trait
use std::cmp::Ordering;

// 多次猜测
fn main() {
    println!("猜数");

    println!("猜测一个数");

    // let mut foo = 1;
    // let bar = foo;// immutabel; 
    // foo = 23;

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop{

        // println!("得到一个随机数 {}", secret_number);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");
        //

        // shadow 类型转换
        // let guess:u32 = guess.trim().parse().expect("please type a number");

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("你的猜测是：{}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("tool small"),
            Ordering::Greater => println!("tool big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}

use std::io; // prelude
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    //功能，猜数游戏
    println!("猜数");
    println!("猜测一个数");
    let secret_number=rand::thread_rng().gen_range(1,101);
    println!("神秘数字{}",secret_number);

    loop{
        let mut guess=String::new(); //关联函数，代表普通编程中的静态方法 static 方法名
        io::stdin().read_line(&mut guess).expect("无法读取"); 
        println!("你猜测的数是：{}",guess);
        // 显示类型u32
        let guess:u32=guess.trim().parse().expect("please type a number!"); //字符串，先去除空格，然后转化成u32
        // 比较大小，
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            },
        }
    }
}

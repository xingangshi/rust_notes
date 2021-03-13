use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[warn(unused_imports)]
fn main() {
    println!("猜数字游戏");

    let aim_number = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("输入你所猜测的数字：");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("获取输入数字失败");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你输入的数字是 {}", guess);

        match guess.cmp(&aim_number) {
            Ordering::Less => println!("你输入的数字小于目标数字"),
            Ordering::Greater => println!("你输入的数字大于目标数字"),
            Ordering::Equal => {
                println!("恭喜你，猜对了，答案是 {}", aim_number);
                break;
            }
        }

    }
}

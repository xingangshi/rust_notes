fn main() {
    let x = 5;
    // x = 6;
    println!("{}", x);
    let mut y = 5;
    println!("{}", y);
    y = 6;
    println!("{}", y);

    let _guess: i32 = 42;
    let _guess = "42".parse::<i32>().expect("Not a number");
    println!("Hello, world!");
    let result = print_from_fn(5, 100);
    println!("result {}", result);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let str1 = "中国人";
    let str2 = String::from(str1);

    for c in str1.chars() {
        println!("{}", c);
    }

    for b in str2.bytes() {
        println!("{}", b);

    }

    println!("condition test {}", if_else_test(10));
    loop_test(10);
}

// 单行注释
// 多行注释，每一行都需要增加 //

fn print_from_fn(x: i32, y: i32) -> i32 {
    println!("hello world, from fun first {}, second {}", x, y);
    x + y
}

fn if_else_test(x : i32) -> String {
    if x < 0 {
        String::from("x 小于 0")
    } else if  x == 0 {
        String::from("x 等于 0")
    } else {
        String::from("x 大于 0")
    }
}

fn loop_test(x : i32) {
    let mut sum = 0;
    let mut index = 0;

    // loop
    let result = loop {
        index += 1;

        if index == x {
            break sum;
        } else {
            sum += index;
        }
    };

    println!("{}", result);
}

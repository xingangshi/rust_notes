fn main() {
    println!("Hello, world!");
    let result = print_from_fn(5, 100);
    println!("result {}", result);

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

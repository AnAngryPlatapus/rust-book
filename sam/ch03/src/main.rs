use std::io;

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn main() {
    // let mut x = 5;
    // // println!("The value of x is: {}", x);
    // x = 6;
    // // println!("The value of x is: {}", x);
    //
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //
    // let x = 5;
    //
    // let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);
    //
    // let mut spaces = "  ";
    // let spaces = spaces.len();
    // println!("The length of spaces is: {}", spaces);
    //
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("The value of guess is: {}", guess);
    //
    // let x = 2.0;
    // let y: f32 = 3.0;
    //
    //
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    //
    // println!("The value of y is: {}", y);
    //
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_4_ = x.1;
    // let one = x.2;
    // println!("The value of five_hundred is: {}", five_hundred);
    //
    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March"];
    //
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("The value of a[0] is: {}", a[0]);
    //
    //
    // let a = [1, 2, 3, 4, 5];
    //
    // println!("Please enter an array index.");
    //
    // let mut index = String::new();
    //
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");
    //
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    //
    // let element = a[index];
    //
    // println!("The value of the element at index {index} is: {element}");

//     another_function(22);
//     print_labeled_measurement(22, 'H');
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {}", y);
//     let x = five();
//     println!("The value of x is: {}", x);
//     let x = plus_one(5);
//     println!("The value of x is: {}", x);
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value iz: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
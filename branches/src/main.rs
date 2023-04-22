fn main() {
    // let number: i32 = 3;
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // let numbe: i32 = 3;
    // if number {
    //     println!("number was three");
    // }

    // let number: i32 = 3;
    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // let number: i32 = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    let condition: bool = true;
    let number: i32 = if condition {
        5
    } else {
        "$ix" // 6
    };
    println!("The value of number is {}", number);
}

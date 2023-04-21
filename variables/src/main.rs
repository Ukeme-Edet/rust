fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    let spaces = "     ";
    let spaces = spaces.len();

    let mut spaces = "     ";
    // spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number");

    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let f = true;
    let t = false;

    let c = 'z';
    let z = 'z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, i32) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, i32) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

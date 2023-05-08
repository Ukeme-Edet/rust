// fn main() {
// S is not valid here
// let _s: String = String::from("hello"); // s is valid from this point foward

// do stuff with s

// let s1: String = String::from("hello");
// let s2: String = s1.clone();
// println!("{}, world!", s1);
// println!("s1 = {}, s2 = {}", s1, s2);

// } // this scope is now over, and s is no longer valid

// fn main() {
//     let s: String = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's value moves into the function...
//                         // ... and is no longer valid here

//     let x: i32 = 5; // x comes into scope

//     makes_copy(x); // x would move into the function,
//                    // but i32 is Copy, so it's okay to
//                    //still use x afterward
// } // Here, x goes out of scope, then s. Bit because s's value was moved,
//   // nothing special happens

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope

//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memeory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope, Nothing special happens

fn main() {
    let s1 = String::from("hello");

    let len: usize = calculate_len(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

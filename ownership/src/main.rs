fn main() {
    // S is not valid here
    let _s: String = String::from("hello"); // s is valid from this point foward

    // do stuff with s

    // let s1: String = String::from("hello");
    // let _s2: String = s1;
    // println!("{}", s1);
} // this scope is now over, and s is no longer valid

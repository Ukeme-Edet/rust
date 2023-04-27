fn main() {                     // S is not valid here
    let _s: &str = "hello";     // s is valid from this point foward

    // do stuff with s
}                               // this scope is now over, and s is no longer valid

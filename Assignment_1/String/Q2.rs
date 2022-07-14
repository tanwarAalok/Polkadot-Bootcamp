
// Fix the error with at least two solutions
fn main() {
    let s: Box<str> =  "hello, world".into();
    // greetings(&s)   // method 1
    greetings(s)       // method 2     
}

fn greetings(s: Box<str>) {
    println!("{}",s)
}

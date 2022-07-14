
// Fix error with at least two solutions
fn main() {
    // let s =  "hello, world".to_string();   // method 1
    let s =  String::from("hello, world");    // method 2
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}

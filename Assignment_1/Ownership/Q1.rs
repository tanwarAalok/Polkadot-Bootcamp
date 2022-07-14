
fn main() {
    // Use as many approaches as you can to make it work
    // Method 1
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
    
    // method 2
    // let x = &String::from("hello, world");
    // let y = x;
    // println!("{},{}",x,y);
    
    // method 3
    // let x = "hello, world";
    // let y = x;
    // println!("{},{}",x,y);
}

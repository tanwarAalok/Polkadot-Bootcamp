
// Modify this struct to make the code work
struct Point<T, A> {
    x: T,
    y: A,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}

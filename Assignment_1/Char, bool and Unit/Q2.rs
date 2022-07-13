
// Make it work
fn main() {
    let c1 = '中';  // char should be in '' not ""
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}

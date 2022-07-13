
fn main() {

    let x : f32 = 0.1;
    let y : f32 = 0.2;
   
    assert!(x+y==0.3_f32);  // method 1
    assert!(0.1_f32+0.2_f32==0.3_f32);  // method 2

    println!("Success!");
}

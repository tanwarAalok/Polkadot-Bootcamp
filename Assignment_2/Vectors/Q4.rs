
// FIX the error and IMPLEMENT the code
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
       if(i < 3){
           v[i] += 1;
       }
       else {
           v.push(i+2);
       }
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}

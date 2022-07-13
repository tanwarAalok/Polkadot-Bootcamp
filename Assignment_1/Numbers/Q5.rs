
// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u16 + 8;                       // changed u8 -> u16
   let v2 = u16::checked_add(251, 8).unwrap(); // changed u8 -> u16
   println!("{},{}",v1,v2);
}

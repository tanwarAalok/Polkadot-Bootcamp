struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {  // using mut to make it mutable
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}


#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {

    fn color(&self) -> String{
        match self {
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
    
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}

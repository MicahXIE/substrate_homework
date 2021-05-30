pub trait Time {
    fn time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 80,
            TrafficLight::Yellow => 100,
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let green_light = TrafficLight::Green;
    let yellow_light = TrafficLight::Yellow;

    println!("red light time is {}", red_light.time());
    println!("green light time is {}", green_light.time());
    println!("yellow light time is {}", yellow_light.time());
}

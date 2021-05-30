pub trait CalculateArea {
    fn calculate_area(&self) -> u8;
}

struct Shape<T> {
    shape: T,
}

struct Round {
    r: u8,
}

struct  Triangle {
    a: u8,
    h: u8,
}

struct Square {
    a: u8,
}

impl CalculateArea for Round {
    fn calculate_area(&self) -> u8 {
        3.14 as u8 * self.r * self.r
    }
}

impl CalculateArea for Triangle {
    fn calculate_area(&self) -> u8 {
        self.a * self.h / 2
    }
}

impl CalculateArea for Square {
    fn calculate_area(&self) -> u8 {
        self.a * self.a
    }
}

fn calculate_shape_area<T: CalculateArea>(shape: &T) {
    println!("the area is {}", shape.calculate_area());
}


fn main() {
    let round = Round {r: 2};
    let triangle = Triangle {a: 2, h: 3};
    let square = Square {a: 2};
    calculate_shape_area(&round);
    calculate_shape_area(&triangle);
    calculate_shape_area(&square);

    let shape = Shape {shape: square};
    calculate_shape_area(&shape.shape);
}

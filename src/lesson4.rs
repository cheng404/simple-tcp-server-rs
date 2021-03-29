use std::f64::consts::PI;

// 作业1
pub enum TrafficLight {
    Red,
    Yellow,
    Green
}

pub trait Duration {
    fn get_duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn get_duration(&self) -> u32 {
        match self {
            TrafficLight::Green => 120,
            TrafficLight::Red => 90,
            TrafficLight::Yellow => 10,
        }
    }
}


// 作业2
pub fn sum_numbers(numbers: &[u32]) -> Option<u32> {
    let mut result: u32 = 0;
    for &n in numbers {
        match result.checked_add(n) {
            Some(val) => result = val,
            None => {
                return None
            }
        }
    }
    Some(result)
}

#[test]
fn test_sum_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_numbers(&nums), Some(10));
    let nums_overflow = [1, 2, std::u32::MAX];
    assert_eq!(sum_numbers(&nums_overflow), None);
}


// 作业3
pub trait Sharp {
    fn get_area(&self) -> f64;
}

struct Circle {
    radius: f64
}

impl Sharp for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Triangle {
    length: f64
}

impl Sharp for Triangle {
    fn get_area(&self) -> f64 {
        self.length * self.length * (3 as f64).sqrt() / 4.0
    }
}

struct Square {
    length: f64
}

impl Sharp for Square {
    fn get_area(&self) -> f64 {
        self.length * self.length
    }
}

fn print_sharp_area<T: Sharp>(sharp: T) -> f64 {
    let result = sharp.get_area();
    println!("the area is: {:#}", result);
    result
}

#[test]
fn test_print_sharp_area() {
    let circle = Circle{ radius: 2.0 };
    print_sharp_area(circle);
    let square = Square { length: 2.0 };
    print_sharp_area(square);
    let triangle = Triangle{ length: 2.0 };
    print_sharp_area(triangle);
}
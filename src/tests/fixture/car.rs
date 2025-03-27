use crate::models::car::Car;

#[allow(dead_code)]
pub fn car_fixture(id: usize) -> Car {
    Car {
        id: id as i32,
        name: String::from("ferrari"),
        color: Some(String::from("black")),
        year: Some(1980),
    }
}

#[allow(dead_code)]
pub fn cars_fixture(num: usize) -> Vec<Car> {
    let mut cars = vec![];
    for i in 1..num + 1 {
        cars.push(car_fixture(i));
    }
    cars
}

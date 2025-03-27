use crate::models::part::{ self, Part };

#[allow(dead_code)]
pub fn part_fixture(id: usize) -> Part {
    Part {
        id: id as i32,
        car_id: 1,
        name: String::from("alternator"),
    }
}

#[allow(dead_code)]
pub fn parts_fixture(num: usize) -> Vec<Part> {
    let mut parts = vec![];
    for i in 1..num + 1 {
        parts.push(part_fixture(i));
    }
    parts
}

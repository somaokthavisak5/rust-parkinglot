use crate::Person;
use rust_parkinglog::{Color, Colorable};

#[derive(Debug)]
pub struct Transport {
    color: Color,
    seat_max: usize,
    seats: Vec<Person>,
    selected: bool,
}

impl Colorable for Transport {
    fn color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Transport {
    pub fn new() -> Transport {
        Transport {
            color: Self::get_random_color(),
            seat_max: 4,
            seats: Vec::new(),
            selected: false,
        }
    }

    pub fn is_full(&self) -> bool {
        self.seats.len() == self.seat_max
    }

    pub fn add_person(&mut self, person: Person) {
        if !self.is_full() {
            self.seats.push(person);
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self) {
        self.selected = false;
    }
}

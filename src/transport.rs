use crate::Person;
use rand::{thread_rng, Rng};
use rust_parkinglog::{Color, Colorable};

#[derive(Debug)]
pub struct Transport {
    color: Color,
    seat_max: usize,
    seats: Vec<Person>,
    selected: bool,
}

impl Colorable for Transport {
    fn get_color_str(&self) -> &'static str {
        match &self.color {
            Color::None => "",
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Red => "Red",
            Color::Yellow => "Yellow",
        }
    }
    fn get_color(&self) -> &Color {
        match &self.color {
            Color::None => &Color::None,
            Color::Blue => &Color::Blue,
            Color::Green => &Color::Green,
            Color::Red => &Color::Red,
            Color::Yellow => &Color::Yellow,
        }
    }

    fn get_rgb(&self) -> (u8, u8, u8) {
        match &self.color {
            Color::None => (0, 0, 0),
            Color::Blue => (0, 0, 255),
            Color::Green => (0, 255, 0),
            Color::Red => (255, 0, 0),
            Color::Yellow => (255, 255, 0),
        }
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn compare_color(&self, other: &impl Colorable) -> bool {
        self.get_color() == other.get_color()
    }

    fn get_random_color() -> Color {
        match thread_rng().gen_range(0..4) {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Red,
            _ => Color::Yellow,
        }
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

use rand::{thread_rng, Rng};
use rust_parkinglog::{Color, Colorable};

#[derive(Debug)]
pub struct Person {
    color: Color,
}

impl Colorable for Person {
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

impl Person {
    pub fn new() -> Person {
        Person {
            color: Self::get_random_color(),
        }
    }
}

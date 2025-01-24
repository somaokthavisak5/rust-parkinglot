use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    None,
}

pub trait Colorable {
    fn color(&self) -> &Color;

    fn set_color(&mut self, color: Color);

    fn get_color_str(&self) -> &'static str {
        match &self.color() {
            Color::None => "",
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Red => "Red",
            Color::Yellow => "Yellow",
        }
    }
    fn get_color(&self) -> &Color {
        match &self.color() {
            Color::None => &Color::None,
            Color::Blue => &Color::Blue,
            Color::Green => &Color::Green,
            Color::Red => &Color::Red,
            Color::Yellow => &Color::Yellow,
        }
    }

    fn get_rgb(&self) -> (u8, u8, u8) {
        match &self.color() {
            Color::None => (0, 0, 0),
            Color::Blue => (0, 0, 255),
            Color::Green => (0, 255, 0),
            Color::Red => (255, 0, 0),
            Color::Yellow => (255, 255, 0),
        }
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

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        match self {
            Color::None => false,
            Color::Blue => match other {
                Color::Blue => true,
                _ => false,
            },
            Color::Green => match other {
                Color::Green => true,
                _ => false,
            },
            Color::Red => match other {
                Color::Red => true,
                _ => false,
            },
            Color::Yellow => match other {
                Color::Yellow => true,
                _ => false,
            },
        }
    }
}

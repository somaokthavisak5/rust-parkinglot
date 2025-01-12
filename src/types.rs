#[derive(Debug)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    None,
}

pub trait Colorable {
    fn get_color_str(&self) -> &'static str;
    fn get_color(&self) -> &Color;
    fn set_color(&mut self, color: Color);
    fn get_rgb(&self) -> (u8, u8, u8);
    fn compare_color(&self, other: &impl Colorable) -> bool;
    fn get_random_color() -> Color;
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

use rust_parkinglog::{Color, Colorable};

#[derive(Debug)]
pub struct Person {
    color: Color,
}

impl Colorable for Person {
    fn color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Person {
    pub fn new() -> Person {
        Person {
            color: Self::get_random_color(),
        }
    }
}

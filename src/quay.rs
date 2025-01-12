use rust_parkinglog::Colorable;

use crate::{person::Person, Transport};

pub struct Quay {
    pub capacity: usize,
    pub parked: Vec<Transport>,
}

impl Quay {
    pub fn new() -> Quay {
        Quay {
            capacity: 4,
            parked: Vec::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.parked.len() == self.capacity
    }

    pub fn park_transport(&mut self, transport: Transport) {
        if !self.is_full() {
            self.parked.push(transport);
        }
    }

    pub fn match_transport_color_with_person(&mut self, person: &Person) -> Vec<&mut Transport> {
        self.parked
            .iter_mut()
            .filter(|t| t.compare_color(person) && !t.is_full())
            .collect()
    }

    pub fn free_full_transport(&mut self) {
        if let Some(pos) = self.parked.iter().position(|t| t.is_full()) {
            println!("Transport is full, {} is leaving", pos);
            self.parked.remove(pos);
        }
    }
}

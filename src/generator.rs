use crate::{parking_lot::ParkingLot, person::Person, transport::Transport};

pub fn generate_parking_lot() -> ParkingLot {
    let mut parking_lot = ParkingLot::new();

    for _ in 0..20 {
        parking_lot.park_transport(Transport::new());
    }

    parking_lot
}

pub fn generate_persons() -> Vec<Person> {
    let mut persons = Vec::new();

    for _ in 0..40 {
        persons.push(Person::new());
    }

    persons
}

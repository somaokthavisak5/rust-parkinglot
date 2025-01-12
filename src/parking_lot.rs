use crate::Quay;
use crate::Transport;

pub struct ParkingLot {
    pub parked: Vec<Transport>,
}

impl ParkingLot {
    pub fn new() -> ParkingLot {
        ParkingLot { parked: Vec::new() }
    }

    pub fn park_transport(&mut self, transport: Transport) {
        self.parked.push(transport);
    }

    pub fn move_transport(&mut self, index: usize, quay: &mut Quay) {
        if !quay.is_full() {
            let transport = self.parked.remove(index);
            quay.park_transport(transport);
            quay.parked
                .iter()
                .for_each(|t| println!("Transport {:?}", t));
        }
    }
}

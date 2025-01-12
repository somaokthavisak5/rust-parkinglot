mod draw;
mod generator;
mod parking_lot;
mod person;
mod quay;
mod transport;

use person::Person;
use quay::Quay;
use transport::Transport;

use draw::{draw_person, draw_quay, draw_transport};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let mut quay = Quay::new();
    let mut generated_parking_lot = generator::generate_parking_lot();
    let mut generated_persons = generator::generate_persons();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Parking Lot", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut selected_transport = 0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    generated_parking_lot.parked[selected_transport].deselect();
                    if selected_transport > 0 {
                        selected_transport -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    generated_parking_lot.parked[selected_transport].deselect();
                    if selected_transport < generated_parking_lot.parked.len() - 1 {
                        selected_transport += 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    println!("Enter pressed {}", selected_transport);
                    generated_parking_lot.move_transport(selected_transport, &mut quay);
                    selected_transport = 0;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        generated_parking_lot.parked[selected_transport].select();

        generated_persons
            .iter()
            .enumerate()
            .for_each(|(index, person)| {
                draw_person(&mut canvas, &person, index).unwrap();
            });

        for pos in 0..quay.capacity {
            draw_quay(
                &mut canvas,
                if quay.parked.len() > pos {
                    Some(&quay.parked[pos])
                } else {
                    None
                },
                pos,
            )
            .unwrap();
        }

        generated_parking_lot
            .parked
            .iter()
            .enumerate()
            .for_each(|(index, transport)| {
                draw_transport(&mut canvas, &transport, index).unwrap();
            });

        //draw_parking_lot(&mut canvas, &generated_parking_lot).unwrap();

        if quay.parked.len() > 0 {
            // check colors the transports in the quay and the first person in the queue
            if generated_persons.len() > 0 {
                let mut transport_available =
                    quay.match_transport_color_with_person(&generated_persons[0]);

                if transport_available.len() > 0 {
                    // if there are matches, move the person to the transport
                    transport_available[0].add_person(generated_persons.remove(0));
                }
            }
            // If the transport are full, remove it from the quay
            quay.free_full_transport();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

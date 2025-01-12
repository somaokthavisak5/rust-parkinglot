use rust_parkinglog::Colorable;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::{parking_lot::ParkingLot, person::Person, transport::Transport};

pub fn draw_transport(
    canvas: &mut Canvas<Window>,
    transport: &Transport,
    position: usize,
) -> Result<(), String> {
    let position_x = position as i32 * 100;

    if transport.is_selected() {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(sdl2::rect::Rect::new(position_x, 300, 20, 30))
            .unwrap();
    }

    let rect = sdl2::rect::Rect::new(position_x, 300, 20, 20);
    canvas.set_draw_color(transport.get_rgb());
    canvas.fill_rect(rect).unwrap();

    Ok(())
}

pub fn draw_parking_lot(
    canvas: &mut Canvas<Window>,
    parkinglot: &ParkingLot,
) -> Result<(), String> {
    let rect = sdl2::rect::Rect::new(30, 200, 300, 300);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(rect).unwrap();

    Ok(())
}

pub fn draw_person(
    canvas: &mut Canvas<Window>,
    person: &Person,
    position: usize,
) -> Result<(), String> {
    let rect = sdl2::rect::Rect::new(position as i32 * 60 + 30, 60, 10, 10);
    canvas.set_draw_color(person.get_rgb());
    canvas.fill_rect(rect).unwrap();

    Ok(())
}

pub fn draw_quay(
    canvas: &mut Canvas<Window>,
    transport: Option<&Transport>,
    position: usize,
) -> Result<(), String> {
    let rect = sdl2::rect::Rect::new(position as i32 * 30 + 30, 200, 20, 20);

    match transport {
        Some(transport) => {
            canvas.set_draw_color(transport.get_rgb());
        }
        None => {
            canvas.set_draw_color(Color::RGB(120, 120, 120));
        }
    };

    canvas.fill_rect(rect).unwrap();

    Ok(())
}

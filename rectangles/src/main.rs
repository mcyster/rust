extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::Rng;

fn draw_circle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, center: Point, radius: i32) {
    for x in 0..=radius {
        for y in 0..=radius {
            if x * x + y * y <= radius * radius {
                canvas.draw_point(Point::new(center.x + x, center.y + y)).unwrap();
                canvas.draw_point(Point::new(center.x + x, center.y - y)).unwrap();
                canvas.draw_point(Point::new(center.x - x, center.y + y)).unwrap();
                canvas.draw_point(Point::new(center.x - x, center.y - y)).unwrap();
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Retangles (<esc> to exit)", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let center = Point::new(400, 300);
    let radius = 100;
    draw_circle(&mut canvas, center, radius);

    let mut random = rand::thread_rng();

    for _ in 0..1000 {
           // Generate random color
           let red: u8 = random.gen_range(0..=255);
           let green: u8 = random.gen_range(0..=255);
           let blue: u8 = random.gen_range(0..=255);

           // Generate random rectangle size and position
           let x = random.gen_range(0..=700);
           let y = random.gen_range(0..=700);
           let width = random.gen_range(50..=200);
           let height = random.gen_range(50..=200);
           let rectangle = Rect::new(x, y, width, height);

           // Set random color and draw rectangle
           canvas.set_draw_color(Color::RGB(red, green, blue));
           canvas.fill_rect(rectangle).unwrap();

           canvas.present();
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

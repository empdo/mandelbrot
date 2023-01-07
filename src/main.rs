extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

pub fn draw_mandel(
    canvas: &mut WindowCanvas,
    width: u32,
    height: u32,
    x_offset: f32,
    y_offset: f32,
    max_it: i32,
    value: f32,
) {
    for x in 0..width {
        for y in 0..height {
            let mut zx = (((x as f32) / (width as f32)) * value) - x_offset;
            let mut zy = (((y as f32) / (height as f32)) * value) - y_offset;

            let cx = zx;
            let cy = zy;

            let mut n = 0;
            while n < max_it {
                let _zx = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = _zx;

                if (zx * zx + zy * zy) > 4.0 {
                    break;
                }
                n += 1;
            }

            if n == max_it {
                n = 255
            }

            let n = n as u8;

            canvas.set_draw_color(Color::RGB(n, n, n));
            canvas.fill_rect(Rect::new(x as i32, y as i32, 1, 1));
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 1200;
    let height = 900;

    let window = video_subsystem
        .window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut x_offset = 2.0;
    let mut y_offset = 0.7;
    let mut max_it = 100;
    let mut value = 2.0;
    let mut interval = 0.1;
    let mut zoom_step = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    y_offset += interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    y_offset -= interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    x_offset -= interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    x_offset += interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    zoom_step += 1;
                    let _value = f32::powf(0.9, zoom_step as f32) * 0.2;

                    if (_value < 0.01) {
                        value -= 0.01;
                    } else {
                        value -= _value;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    zoom_step -= 1;
                    value += f32::powf(0.9, zoom_step as f32) * 0.2;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        draw_mandel(
            &mut canvas,
            width,
            height,
            x_offset,
            y_offset,
            max_it,
            value,
        );

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

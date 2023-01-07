extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;

pub fn draw_mandel(canvas: & mut WindowCanvas, width: i32, height: i32, x_offset: f32, y_offset: f32, max_it: i32, value: f32) {
    
    for x in 0..width  {
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
            canvas.fill_rect(Rect::new(x, y, 1, 1));
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();


    let width = 800;
    let height = 600;
    let mut x_offset = 2.0;
    let mut y_offset = 0.7;
    let mut max_it = 100;
    let mut value = 2.0;
    let mut interval = 0.1;


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                     y_offset += interval;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                     y_offset -= interval;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                     x_offset -= interval;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                     x_offset += interval;
                },
                Event::KeyDown { keycode: Some(Keycode::Plus), .. } => {
                     value -= interval;
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                     value += interval;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        
        draw_mandel(&mut canvas, width, height, x_offset, y_offset, max_it, value);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

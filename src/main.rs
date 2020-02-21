extern crate sdl2;

use std::path::Path;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let texture = texture_creator.load_texture(Path::new("assets/characters.png"))?;

    let frames_per_anim = 4;
    let sprite_tile_size = (32,32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_0.center_on(Point::new(-64,120));

    // King - walk animation
    let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_1.center_on(Point::new(0,240));

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_2.center_on(Point::new(440,360));

    let mut timer = sdl_context.timer()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut running = true;
    let mut going_left = true;
    let mut frame = 0;
    while running {
        let start_tick = timer.ticks();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    going_left = true;
                }
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    going_left = false;
                }
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }


        // set the current frame for time
        source_rect_0.set_x(32 * frame);
        dest_rect_0.set_x((dest_rect_0.x() + 5) % 768);

        source_rect_1.set_x(32 * frame);
        let mut x = dest_rect_1.x();
        if going_left {
            x -= 6;
            if x < -128 {
                x = 608;
            }
        } else {
            x += 6;
            if x > 608 {
                x = -128;
            }
        }
        dest_rect_1.set_x( x );

        source_rect_2.set_x(32 * frame);
        dest_rect_2.set_x( (dest_rect_2.x() + 7) % 768);

        canvas.clear();
        // copy the frame to the canvas
        canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false)?;
        canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, going_left, false)?;
        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false)?;
        canvas.present();

        let stop_tick = timer.ticks();
        let frame_time = stop_tick - start_tick;

        if frame_time < 100 {
            let sleep_time = (100 - frame_time) as u64;
            if sleep_time > 1 {
                std::thread::sleep(Duration::from_millis(sleep_time));    
            }
        } else {
            println!("BIG frame time: {}", frame_time);
        }
        frame = (frame + 1) % frames_per_anim;
    }

    Ok(())
}


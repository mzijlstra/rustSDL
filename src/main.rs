extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::path::Path;
use std::time::Duration;

struct Player {
    source: Rect,
    dest: Rect,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    up_count: u32,
    down_count: u32,
    flame: i32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            source: Rect::new(0,0,0,0),
            dest: Rect::new(0,0,0,0),
            up: false,
            down: false,
            left: false,
            right: false,        
            up_count: 0,
            down_count: 0,
            flame: 0,
        }
    }
}

fn main() -> Result<(), String> {
    let window_size: (i32, i32) = (480, 300);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("SDL2", window_size.0 as u32, window_size.1 as u32)
        .position_centered()
        //.fullscreen()
        .build()
        .map_err(|e| e.to_string())?;


    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

    // sprites and background form: https://opengameart.org/content/space-ship-shooter-pixel-art-assets
    let ship_texture = texture_creator.load_texture(Path::new("assets/ship-test.png"))?;
    let bg_texture = texture_creator.load_texture(Path::new("assets/desert-background-test.png"))?;

    // background
    let source_bg = Rect::new(0, 0, 272, 300);
    let mut dest_bg_array: [Rect; 3] = [
        Rect::new(0, 0, 272, 300),
        Rect::new(272, 0, 272, 300),
        Rect::new(544, 0, 272, 300),
    ];

    // ship related
    let frames_per_anim = 2;
    let sprite_tile_size = (24, 16);

    let mut player = Player {
        source: Rect::new(24, 32, sprite_tile_size.0, sprite_tile_size.1),
        dest: Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.1),
        ..Default::default()
    };
    player.dest.center_on(Point::new(window_size.0 / 4, window_size.1 / 2));

    let mut timer = sdl_context.timer()?;
    let mut time = timer.ticks();
    let mut second = time + 1000;
    let mut frame_count = 0;

    let mut event_pump = sdl_context.event_pump()?;
    let mut running = true;
    while running {
        let start_tick = timer.ticks();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } | Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                }=> {
                    if !player.up {
                        player.up = true;
                        player.down = false;
                        player.up_count = 0;    
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } | Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    player.up = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } | Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if !player.down {
                        player.down = true;
                        player.up = false;
                        player.down_count = 0;    
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } | Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    player.down = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } | Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player.right = false;
                    player.left = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } | Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player.left = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } | Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player.left = false;
                    player.right = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } | Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player.right = false;
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        // move background parts
        for dest_bg in &mut dest_bg_array {
            let mut bg_x = dest_bg.x();
            if bg_x - 1 < -272 {
                bg_x = 544;
            }
            dest_bg.set_x(bg_x - 1);
        }

        // move ship
        let mut ship_x = player.dest.x();
        let mut ship_y = player.dest.y();
        let mut ship_tilt = 32;
        if player.up {
            // move ship
            let delta_y = -1;
            if ship_y + delta_y >= -8 {
                ship_y = ship_y + delta_y;
            }
            // tilt ship
            player.up_count += 1;
            if player.up_count > 15 {
                ship_tilt = 0;
            } else {
                ship_tilt = 16;
            }
        } else if player.down {
            // move ship
            let delta_y = 1;
            if ship_y + delta_y < 8 + window_size.1 - sprite_tile_size.1 as i32 {
                ship_y = ship_y + delta_y;
            }
            // tilt ship
            player.down_count += 1;
            if player.down_count > 15 {
                ship_tilt = 64;
            } else {
                ship_tilt = 48;
            }
        }
        
        if player.right {
            // animate exhaust flame
            if frame_count % 5 == 0 {
                player.flame = (player.flame + 1) % frames_per_anim;
            }
            let delta_x = 1;
            if ship_x + delta_x < window_size.0 - sprite_tile_size.0 as i32 {
                ship_x += delta_x;
            }
        } else if player.left {
            let delta_x = -1;
            if ship_x + delta_x > 0 {
                ship_x += delta_x;
            }
            player.flame = 3;
        } else {
            player.flame = 2;
        }
        player.source.set_y(ship_tilt);
        player.source.set_x(24 * player.flame);
        player.dest.set_x(ship_x);
        player.dest.set_y(ship_y);

        // draw on screen
        canvas.clear();
        for dest_bg in &dest_bg_array {
            canvas.copy_ex(
                &bg_texture,
                Some(source_bg),
                Some(*dest_bg),
                0.0,
                None,
                false,
                false,
            )?;
        }
        canvas.copy_ex(
            &ship_texture,
            Some(player.source),
            Some(player.dest),
            0.0,
            None,
            false,
            false,
        )?;
        canvas.present();

        time += 10;
        while timer.ticks() < time {
            std::thread::sleep(Duration::from_millis(1));
        }
        frame_count += 1;
        if timer.ticks() > second {
            second += 1000;
            println!("frames: {}", frame_count);
            frame_count = 0;
        }
        let stop_tick = timer.ticks();
        if stop_tick - start_tick > 12 {
            println!("big frame size: {}", stop_tick - start_tick);
        }
    }

    Ok(())
}

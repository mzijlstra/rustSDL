extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::video::FullscreenType;
use sdl2::surface::Surface;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

struct Player {
    source: Rect,
    dest: Rect,
    flame_source: Rect,
    flame_dest: Rect,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    up_count: u32,
    down_count: u32,
    right_count: i32,
    flame: i32,
    tilt: i32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            source: Rect::new(0, 0, 0, 0),
            dest: Rect::new(0, 0, 0, 0),
            flame_source: Rect::new(0, 0, 0, 0),
            flame_dest: Rect::new(0, 0, 0, 0),
            up: false,
            down: false,
            left: false,
            right: false,
            up_count: 0,
            down_count: 0,
            right_count: 0,
            flame: 2,
            tilt: 32,
        }
    }
}

fn main() -> Result<(), String> {
    let window_desired_size: (i32, i32) = (640, 360);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window(
            "SDL2",
            window_desired_size.0 as u32,
            window_desired_size.1 as u32,
        )
        .position_centered()
        //.fullscreen()
        .build()
        .map_err(|e| e.to_string())?;

    let window_actual_size: (u32, u32) = window.size();
    println!(
        "window size: {},{}",
        window_actual_size.0, window_actual_size.1
    );

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let tile_size = (16, 16);

    // sprites form: https://opengameart.org/content/space-ship-shooter-pixel-art-assets
    let ship_texture = texture_creator.load_texture(Path::new("assets/ship-sheet.png"))?;
    let flame_texture = texture_creator.load_texture(Path::new("assets/flame-sheet.png"))?;
    let bg_tiles = texture_creator.load_texture(Path::new("assets/desert_tiles.png"))?;
    let mut bg_canvas = Surface::new(1280,720).into_canvas();

    // background
    let source_bg = Rect::new(0, 0, tile_size.0, tile_size.1);
    let dest_bg = Rect::new(0, 0, tile_size.0, tile_size.1);

    // ship related
    let mut player = Player {
        source: Rect::new(32, 0, tile_size.0, tile_size.1),
        dest: Rect::new(0, 0, tile_size.0, tile_size.1),
        flame_source: Rect::new(0, 0, tile_size.0, tile_size.1),
        flame_dest: Rect::new(0, 0, tile_size.0, tile_size.1),
        ..Default::default()
    };
    player.dest.center_on(Point::new(
        window_actual_size.0 as i32 / 4,
        window_actual_size.1 as i32 / 2,
    ));

    let normal_frame = Duration::from_nanos(16666667); // makes 60 fps
    let mut frame_count: u128 = 0;
    let mut fps: u8 = 0;
    let mut secs = 0;
    let mut event_pump = sdl_context.event_pump()?;
    let mut running = true;
    let start_time = Instant::now();
    while running {
        let frame_time = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    if !player.up {
                        player.up = true;
                        player.down = false;
                        player.up_count = 0;
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    player.up = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::KeyDown {
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
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    player.down = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player.right = false;
                    player.left = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player.left = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    if !player.right {
                        player.left = false;
                        player.right = true;
                        player.right_count = 0;
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyUp {
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
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => match canvas.window().fullscreen_state() {
                    FullscreenType::Off => {
                        canvas.window_mut().set_fullscreen(FullscreenType::True)?;
                    }
                    FullscreenType::True => {
                        canvas.window_mut().set_fullscreen(FullscreenType::Off)?;
                    }
                    FullscreenType::Desktop => {
                        canvas.window_mut().set_fullscreen(FullscreenType::Off)?;
                    }
                },
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
        if player.up {
            let delta_y = -1;
            if ship_y + delta_y >= -8 {
                ship_y = ship_y + delta_y;
            }
            player.up_count += 1;
        } else if player.up_count > 0 {
            player.up_count -= 1;
        }

        if player.down {
            let delta_y = 1;
            if ship_y + delta_y < 8 + window_actual_size.1 as i32 - tile_size.1 as i32 {
                ship_y = ship_y + delta_y;
            }
            player.down_count += 1;
        } else if player.down_count > 0 {
            player.down_count -= 1;
        }

        // tilt ship up or down
        if (player.up_count > 0 && player.down_count > 0)
            || (player.up_count == 0 && player.down_count == 0)
        {
            player.tilt = 2;
            if player.up {
                player.down_count /= 2;
            } else if player.down {
                player.up_count /= 2;
            }
        } else if player.up_count < 16 && player.down_count == 0 {
            player.tilt = 1;
        } else if player.up_count >= 16 {
            player.tilt = 0;
            player.up_count = 16;
        } else if player.up_count == 0 && player.down_count < 16 {
            player.tilt = 3;
        } else if player.down_count >= 16 {
            player.tilt = 4;
            player.down_count = 16;
        }

        if player.right {
            // animate exhaust flame
            if player.right_count < 6 {
                player.flame = 2;
            } else if player.right_count < 12 {
                player.flame = 3;
            } else {
                player.right_count = 0;
            }
            player.right_count += 1;
            let delta_x = 1;
            if ship_x + delta_x < window_actual_size.0 as i32 - tile_size.0 as i32 {
                ship_x += delta_x;
            }
        } else if player.left {
            let delta_x = -1;
            if ship_x + delta_x > 0 {
                ship_x += delta_x;
            }
            player.flame = 0;
        } else {
            player.flame = 1;
        }
        player.source.set_x(player.tilt * tile_size.0 as i32);
        player
            .flame_source
            .set_x(player.flame * tile_size.0 as i32);
        player.dest.set_x(ship_x);
        player.dest.set_y(ship_y);
        player.flame_dest.set_x(ship_x - 9);
        player.flame_dest.set_y(ship_y);

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
            &flame_texture,
            Some(player.flame_source),
            Some(player.flame_dest),
            0.0,
            None,
            false,
            false,
        )?;
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

        let target_frames = start_time.elapsed().as_micros() / 16667;
        // only pause if we're on track for 60fps
        if frame_count >= target_frames {
            while frame_time.elapsed() < normal_frame {
                std::thread::sleep(Duration::from_micros(100));
            }
        }
        frame_count += 1;
        fps += 1;

        if start_time.elapsed().as_secs() > secs {
            secs = start_time.elapsed().as_secs();
            println!("fps: {}", fps);
            fps = 0;
        }
        if frame_time.elapsed() > Duration::from_millis(18) {
            println!("big frame size: {}", frame_time.elapsed().as_millis());
        }
    }

    Ok(())
}


extern crate sdl2;
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
 
use std::env;
use std::fs;
use std::io::prelude::*;


use std::process;
use sdl2::rect::{Rect};

use sdl2::rect::Point;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Serialize,Deserialize,Debug)]
struct Game {
    FPS: i32,
    width:i32,
    height:i32
 
}

fn main() {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2",1920,1080)
        .position_centered().build().unwrap();

    let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(9,7,58,1));
    canvas.set_draw_color(sdl2::pixels::Color::RGB(66, 71, 79));

    let mut timer = sdl_context.timer().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("/home/wolf/Documents/rust/game/src/assets/characters.bmp")).unwrap();

    let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

    let frames_per_anim = 4;
    let sprite_tile_size = (32,32);

    //sdl2::timer::Timer

    let FPS = 60 as i32;
    let frameDelay = 1000/ FPS as i32;

    


    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_2.center_on(Point::new(440,860));
    let mut counter = 0;
    let mut positionX = 117;
    let mut running = true;
    let mut direction = false;
    let mut screancount = 1;
    while running {
       
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    positionX = positionX -5;
                    direction = true;
                },
                
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    positionX =  positionX +5;
                    direction = false;
                    
                            //dest_rect_2.set_x();
                 },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                  
                  
                  
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                  
                },
                _ => {}
            }
        }



        let ticks = timer.ticks() as i32;
        
      //  let timer = Timer::new();
        //let ticks = timer.interval_ms(100).iter();
        //println!("Ticks:{}",ticks);
        let myVal = 32 * ((ticks / 100) % frames_per_anim);
        let myValEnd = 1 * ((ticks / 10) % 768) - 128;
        //println!("start:{}",myVal);
        //println!("End:{}\n",1 * ((ticks / 10) % 768) - 128);
        source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
         dest_rect_2.set_x( positionX);

        canvas.clear();
        // copy the frame to the canvas
       
     //   canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false).unwrap();
        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, direction, false).unwrap();
        canvas.present();
//std::thread::sleep(Duration::from_millis(10));
        let frameTime = timer.ticks() as i32;
        let frameTime = frameTime - ticks;
        println!("FrameTime = :{}",frameTime);
        

        if frameDelay > frameTime{
            let sleeptime = (frameDelay - frameTime) as u64;
        std::thread::sleep(Duration::from_millis(sleeptime));
        }

    }
}

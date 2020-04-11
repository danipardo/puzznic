extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::path::Path;

// use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
//use sdl2::rect::Point;
//use std::time::Duration;

mod game;


fn handle_events(event_pump: &mut sdl2::EventPump ) -> bool {

    let mut r: bool = false;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    r=true
            },
            _ => {}
        }
    }

    r
}


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(180, 180, 180));



    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
//    let mut array: [[u8;10 ];10];
//    array[0][0] = 1;

//    println!("{}", array);
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("img/bg1.bmp")).unwrap();
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator.create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string()).unwrap();


    let source_rect = Rect::new(0, 0, 12, 10);

    let mut finished = false;
    let level1 = "
    x - - - - - - -
    - - x x x x x -
    - x x x x x E -
    - x x x x G X -
    - x x x B P C -
    - x x G P C D -
    - x E B X D - -
    - - - - - - - -
  ";

  let level1 = level1.replace(" ",  "");
//  println!("{}", level1);
    
  let tilemap = game::load_map(&level1);



  for x in tilemap.map{
      for title in x {
        let c = title.c;
        print!("{}", c);
      }
      println!("");
  }
    const WIDTH :u32 = 16;
    const HEIGHT :u32 = 8;
    const scale :u32 = 2;

    // let size = array.len();

     while !finished {

        canvas.clear();

        for x  in 0..25 {
            for y in 0..38 {

                let dest_rect = Rect::new( (x*WIDTH*scale) as i32,
                    (y*HEIGHT*scale) as i32,
                    WIDTH *scale,
                    HEIGHT * scale);
                
                    canvas.copy_ex(&texture,
                    Some(source_rect),
                     Some(dest_rect),
                     0.0, None, false, false).unwrap();
      
            }
        }

        canvas.present();
 
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        finished = handle_events(&mut event_pump);

            
        
    }
}


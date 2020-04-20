use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::path::Path;
use sdl2::render::TextureCreator;

const scale: u32 = 2;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub c: char
}

pub struct TileMap {
    pub map: Vec<Vec<Tile>>
}

pub struct GraphicsSubsystem {

    pub canvas: Canvas<Window>,
    pub brick_texture: Option<Texture<'static>>,
    pub event_pump: sdl2::EventPump,
    pub texture_creator: TextureCreator<sdl2::video::WindowContext>,
}

impl GraphicsSubsystem {
    pub fn finished(&self) -> bool {
        return false;
    }
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn get_brick_texture(&self) -> Texture {
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("img/bg1.bmp")).unwrap();


         let brick_texture = self.texture_creator.create_texture_from_surface(&temp_surface).unwrap();

         brick_texture
     }
}

pub fn init() -> GraphicsSubsystem {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("img/bg1.bmp")).unwrap();
    let canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
//
//     // init the backgorund bricks
//     let texture_creator = canvas.texture_creator();
//

    //   let brick_texture =  texture_creator.create_texture_from_surface(&temp_surface).unwrap();
//
//
    let event_pump = sdl_context.event_pump().unwrap();

    GraphicsSubsystem {
        canvas: canvas,
        event_pump: event_pump,
        brick_texture: None,
        texture_creator: texture_creator,
    }
}

pub fn load_map(level: &String) -> TileMap {
    let mut map: Vec<Vec<Tile>> = vec![];

    let lines = level.split("\n");

    for line in lines {
        let mut s1 = vec![];
        for c in line.chars() {
            let t: Tile = Tile { c };

            s1.push(t);
        }

        map.push(s1);
    }
    TileMap { map }
}


pub fn draw_background_tiles<'a>(gs: &GraphicsSubsystem) {
    const WIDTH: u32 = 16;
    const HEIGHT: u32 = 8;


    //  let texture = gs.brick_texture;
    let source_rect = Rect::new(0, 0, 12, 10);

    let texture = gs.get_brick_texture();

    let canvas = &gs.canvas;

    for x in 0..25 {
        for y in 0..38 {
            let dest_rect = Rect::new((x * WIDTH * scale) as i32,
                                      (y * HEIGHT * scale) as i32,
                                      WIDTH * scale,
                                      HEIGHT * scale);

             canvas.copy_ex(&texture,
             Some(source_rect),
              Some(dest_rect),
              0.0, None, false, false).unwrap();
        }
    }
}
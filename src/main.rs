extern crate sdl2;
extern crate rand;
extern crate core;

mod tricks;
mod widgets;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture};
use widgets::{Widget, ParallaxLayer, ParallaxBg};

pub fn main() {
    // Setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Keep the space bar pressed to move the parallax background", 1250, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    let mut layers = vec![];
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/base.png").unwrap());
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/A4.png").unwrap());
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/A3.png").unwrap());
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/A2.png").unwrap());
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/A1.png").unwrap());
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/B3.png").unwrap());
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/B2.png").unwrap());
    layers.push(canvas.texture_creator().load_texture("resources/png/bg/B1.png").unwrap());

    let increments = vec![1, 16, 8, 4, 2, 5, 10, 20];
    let parallax_layers: Vec<ParallaxLayer> = layers.into_iter().zip(increments)
        .map( |(texture, increment)| {
        widgets::ParallaxLayer::new(texture, increment)})
        .collect();

    let mut background = ParallaxBg {layers: parallax_layers};

    canvas.clear();
    background.render(&mut canvas);
    canvas.present();

    'mainloop: loop {

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    canvas.clear();
                    background.render(&mut canvas);
                    canvas.present();
                }
                _ => {  }
            }
        }
    }
}
#![allow(unused)]

use egui::*;
use egui_winit_platform::*;
use winit::
{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::*,
};
use std::fs::File;
use std::io::*;
use image::*;
use anyhow::Result;

fn main() 
{
    println!("Hello, world!");
    convert_to_png("test.webp");
}

fn run_ui()
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut ctx: Context = Context::default();
    let mut platform: Platform = Platform::new(PlatformDescriptor 
        {
            physical_width: window.inner_size().width,
            physical_height: window.inner_size().height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default()
        });

        event_loop.run(move |event, _, control_flow| 
            {
            // handle events
            match event 
            {
                Event::MainEventsCleared =>
                {
                    todo!()
                }
                Event::WindowEvent { window_id, event } =>
                {
                    todo!()
                }
                Event::RedrawRequested(_) =>
                {
                    todo!()
                }
                _ => {}
            }
        });
        
}

fn convert_to_png(webp_path: &str) -> Result<(), anyhow::Error>
{
    //PNG path ska vara samma som webp but .png instead of .webp
    let mut webp_file = File::open(webp_path)?;
    let mut buf_reader = BufReader::new(webp_file);

    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;

    let format = image::guess_format(&buffer)?;

    if format == ImageFormat::WebP 
    {
        let image = image::load_from_memory(&buffer)?;
        let png_data = image.to_rgba8().into_vec();
        
        // Replace the original file with the converted PNG image
        let mut file = File::create(format!("{}.png", webp_path.to_string()))?;
        file.write_all(&png_data);
    }
    todo!()
}
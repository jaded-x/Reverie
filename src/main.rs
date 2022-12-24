pub mod vulkan;

use std::time::Instant;

use vulkan::{app::*, vertex::Vertex, renderable::Renderable};

use winit::event::WindowEvent;

const WINDOW_TITLE: &'static str = "Reverie";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = VulkanApp::new()?;
    let event_loop = app.window.acquire_event_loop()?;

    let mut now = Instant::now();
    let mut avg_fps = 0.0;

    simple_logger::SimpleLogger::new().env().init().unwrap();

    let renderable1 = Renderable::new(&app.device, &mut app.allocator, 4, 6)
        .expect("Failed to create renderable");
    app.renderables.push(renderable1);

    event_loop.run(move |event, _, controlflow| match event {
        winit::event::Event::WindowEvent {event, .. } => match event {
            WindowEvent::CloseRequested => {
                *controlflow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                println!("Window resized to {}px x {}px", size.width, size.height);
            }
            _ => {}
        }
        winit::event::Event::MainEventsCleared => {
            app.window.window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let delta_time = now.elapsed().as_secs_f32() * 1000.0;
            now = Instant::now();
            let fps = ((1000.0/delta_time) * 10.0).round() / 10.0;
            avg_fps = (avg_fps + fps) / 2.0;
            app.set_window_title(&format!("{} - FPS: {:.0} ({:.3}ms) | AVG FPS: {:.0}", WINDOW_TITLE, fps.round(), delta_time, avg_fps.round()));
            

            let vertices: [Vertex; 4] = [
                Vertex {
                    pos: [-0.5, -0.5, 0.0, 1.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                Vertex {
                    pos: [0.5, -0.5, 0.0, 1.0],
                    color: [0.0, 1.0, 0.0, 1.0],
                },
                Vertex {
                    pos: [0.5, 0.5, 0.0, 1.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                },
                Vertex {
                    pos: [-0.5, 0.5, 0.0, 1.0],
                    color: [1.0, 1.0, 1.0, 1.0],
                },
            ];

            let indices: [u32; 6] = [
                0, 1, 2,
                2, 3, 0
            ];

            app.renderables.get_mut(0).unwrap().update_vertices_buffer(&vertices);
            app.renderables.get_mut(0).unwrap().update_indices_buffer(&indices);

            VulkanApp::fill_commandbuffers(&app.commandbuffers, &app.device, &app.renderpass, &app.swapchain, &app.pipeline, &app.renderables)
                .expect("Failed to write commands!");

            app.draw_frame();
        }
        _ => {}
    });
}

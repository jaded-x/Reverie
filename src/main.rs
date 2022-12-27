pub mod vulkan;

use std::time::Instant;

use vulkan::{renderer::*, vertex::Vertex, object::Object, window::VulkanWindow};

use winit::event::WindowEvent;

const WINDOW_TITLE: &'static str = "Reverie";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_loop, window) = VulkanWindow::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let mut renderer = VulkanRenderer::new(&window)?;

    let mut now = Instant::now();

    let object1 = Object::new(&renderer.device, &mut renderer.allocator, 4, 6)
        .expect("Failed to create object");
    renderer.objects.push(object1);


    event_loop.run(move |event, _, controlflow| match event {
        winit::event::Event::WindowEvent {event, .. } => match event {
            WindowEvent::CloseRequested => {
                *controlflow = winit::event_loop::ControlFlow::Exit;
            }
            _ => {}
        }
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let delta_time = now.elapsed().as_secs_f32() * 1000.0;
            now = Instant::now();
            let fps = ((1000.0 / delta_time) * 10.0).round() / 10.0;

            window.set_title(&format!("{} - FPS: {:.0} ({:.3}ms)",
                WINDOW_TITLE, fps.round(), delta_time));

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

            renderer.objects.get_mut(0).unwrap().update_vertices_buffer(&vertices);
            renderer.objects.get_mut(0).unwrap().update_indices_buffer(&indices);

            VulkanRenderer::fill_commandbuffers(&renderer.commandbuffers, &renderer.device, &renderer.renderpass, &renderer.swapchain, &renderer.pipeline, &renderer.objects)
                .expect("Failed to write commands!");

            renderer.draw_frame();
        }
        _ => {}
    });
}

pub mod vulkan;
pub mod utils;

use std::time::Instant;

use vulkan::{renderer::*, vertex::Vertex, mesh::Mesh, window::VulkanWindow, game_object::GameObject};

use winit::event::WindowEvent;

const WINDOW_TITLE: &'static str = "Reverie";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_loop, window) = VulkanWindow::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let mut renderer = VulkanRenderer::new(&window)?;

    let mut now = Instant::now();
    
    let mut mesh1 = Mesh::new(&renderer.device, &mut renderer.allocator, 4, 6)?;

    let vertices: [Vertex; 4] = [
        Vertex {
            pos: uv::Vec2::new(-0.5, -0.5),
            color: uv::Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            pos: uv::Vec2::new(0.5, -0.5),
            color: uv::Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            pos: uv::Vec2::new(0.5, 0.5),
            color: uv::Vec3::new(0.0, 0.0, 1.0),
        },
        Vertex {
            pos: uv::Vec2::new(-0.5, 0.5),
            color: uv::Vec3::new(1.0, 1.0, 1.0),
        },
    ];

    let indices: [u32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    mesh1.vertex_buffers[0].update_buffer(&vertices);
    mesh1.update_index_buffer(&indices);

    let mut square = GameObject::new(mesh1, uv::Vec3::new(0.0, 0.0, 1.0));
    square.transform2d.translation.x = 0.2;

    renderer.game_objects.push(square);

    event_loop.run(move |event, _, controlflow| match event {
        winit::event::Event::WindowEvent {event, ..} => match event {
            WindowEvent::CloseRequested => {
                *controlflow = winit::event_loop::ControlFlow::Exit;
            }
            _ => {}
        }
        winit::event::Event::MainEventsCleared => {
            window.window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let delta_time = now.elapsed().as_secs_f32() * 1000.0;
            now = Instant::now();
            let fps = ((1000.0 / delta_time) * 10.0).round() / 10.0;

            window.window.set_title(&format!("{} - FPS: {:.0} ({:.3}ms)",
                WINDOW_TITLE, fps.round(), delta_time));

            VulkanRenderer::fill_commandbuffers(&renderer.command_buffers, &renderer.device, &renderer.renderpass, &renderer.swapchain, &renderer.pipeline, &renderer.game_objects)
                .expect("Failed to write commands!");

            renderer.draw_frame();
        }
        _ => {}
    });
}
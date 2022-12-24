pub mod vulkan;

use vulkan::app::VulkanApp;

use winit::event::WindowEvent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = VulkanApp::new()?;
    let event_loop = app.window.acquire_event_loop()?;

    event_loop.run(move |event, _, controlflow| match event {
        winit::event::Event::WindowEvent {event, .. } => match event {
            WindowEvent::CloseRequested => {
                *controlflow = winit::event_loop::ControlFlow::Exit;
            }
            _ => {}
        }
        _ => {}
    });
}

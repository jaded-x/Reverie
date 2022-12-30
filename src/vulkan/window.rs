use winit::event_loop::EventLoop;
use winit::window::Window;

use anyhow::Result;

pub struct VulkanWindow {
    pub window: Window,
    pub width: u32,
    pub height: u32
}

impl VulkanWindow {
    pub fn create_window(title: &'static str, width: u32, height: u32) -> Result<(EventLoop<()>, Self)> {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Failed to create window.");

        Ok((event_loop, Self {
                window,
                width,
                height
        }))
    }
}
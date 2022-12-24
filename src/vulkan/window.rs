use winit::event_loop::EventLoop;
use winit::window::Window;

use anyhow::Result;

pub struct RendererWindow {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
}

impl RendererWindow {
    pub fn create_window(title: &'static str, width: u32, height: u32) -> Result<(EventLoop<()>, Window)> {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Failed to create window.");

        Ok((event_loop, window))
    }

    pub fn new(event_loop: EventLoop<()>, window: Window, entry: &ash::Entry, instance: &ash::Instance
    ) -> Result<RendererWindow> {
        Ok(RendererWindow {
            event_loop: Some(event_loop),
            window,
        })
    }


    pub fn acquire_event_loop(&mut self) -> Result<EventLoop<()>> {
        match self.event_loop.take() {
            None => anyhow::bail!("EventLoop was acquired before."),
            Some(e) => Ok(e)
        }
    }

    pub unsafe fn cleanup(&self) {}
}
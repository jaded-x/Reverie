use ash::vk;

use winit::event_loop::EventLoop;
use winit::window::Window;

use anyhow::Result;

pub struct VulkanWindow {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: ash::extensions::khr::Surface,
}

impl VulkanWindow {
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
    ) -> Result<Self> {
        let surface = unsafe { ash_window::create_surface(&entry, &instance, &window, None).unwrap() };
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        Ok(Self {
            event_loop: Some(event_loop),
            window,
            surface,
            surface_loader
        })
    }

    pub fn get_capabilities(&self, physical_device: vk::PhysicalDevice) -> Result<vk::SurfaceCapabilitiesKHR, vk::Result> {
        unsafe {
            self.surface_loader.get_physical_device_surface_capabilities(physical_device, self.surface)
        }
    }

    pub fn get_present_modes(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::PresentModeKHR>, vk::Result> {
        unsafe {
            self.surface_loader.get_physical_device_surface_present_modes(physical_device, self.surface)
        }
    }

    pub fn get_formats(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::SurfaceFormatKHR>, vk::Result> {
        unsafe {
            self.surface_loader.get_physical_device_surface_formats(physical_device, self.surface)
        }
    }

    pub fn get_physical_device_surface_support(&self, physical_device: vk::PhysicalDevice, queue_family_index: usize) -> Result<bool, vk::Result> {
        unsafe {
            self.surface_loader.get_physical_device_surface_support(physical_device, queue_family_index as u32, self.surface)
        }
    }
 
    pub fn acquire_event_loop(&mut self) -> Result<EventLoop<()>> {
        match self.event_loop.take() {
            None => anyhow::bail!("EventLoop was acquired before."),
            Some(e) => Ok(e)
        }
    }

    pub unsafe fn cleanup(&mut self) {
        self.surface_loader.destroy_surface(self.surface, None);
    }
}
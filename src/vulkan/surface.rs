use ash::vk;

pub struct VulkanSurface {
    pub surface: vk::SurfaceKHR,
    pub surface_loader: ash::extensions::khr::Surface
}

impl VulkanSurface {
    pub fn new(window: &winit::window::Window, entry: &ash::Entry, instance: &ash::Instance
    ) -> Result<Self, vk::Result> {
        let surface = unsafe { ash_window::create_surface(&entry, &instance, &window, None).unwrap() };
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        Ok(Self {
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

    pub unsafe fn cleanup(&mut self) {
        self.surface_loader.destroy_surface(self.surface, None);
    }
}
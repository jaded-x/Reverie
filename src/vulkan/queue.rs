use ash::vk;
use super::window::VulkanWindow;

pub struct QueueFamilies {
    pub graphics: Option<u32>,
    pub transfer: Option<u32>
}

impl QueueFamilies {
    pub fn new(instance: &ash::Instance, physical_device: vk::PhysicalDevice, window: &VulkanWindow) -> Result<QueueFamilies, vk::Result> {
        let mut queue_families = QueueFamilies {
            graphics: None,
            transfer: None,
        };

        let queue_family_properties = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let mut found_graphics_queue_index = None;
        let mut found_transfer_queue_index = None;
        
        for (index, queue_family) in queue_family_properties.iter().enumerate() {
            if queue_family.queue_count > 0 && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
                unsafe { window.surface_loader.get_physical_device_surface_support(physical_device, index as u32, window.surface).unwrap() } {
                    found_graphics_queue_index = Some(index as u32);
                }
            if queue_family.queue_count > 0 && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                if found_transfer_queue_index.is_none() || !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    found_transfer_queue_index = Some(index as u32);
                }
            }
        }

        queue_families.graphics = found_graphics_queue_index;
        queue_families.transfer = found_transfer_queue_index;

        Ok(queue_families)
    }
}
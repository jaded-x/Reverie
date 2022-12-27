use ash::vk;
use super::queue::QueueFamilies;

pub struct Pools {
    pub graphics_command_pool: vk::CommandPool,
    pub transfer_command_pool: vk::CommandPool,
}

impl Pools {
    pub fn new(logical_device: &ash::Device, queue_families: &QueueFamilies) -> Result<Pools, vk::Result> {
        let graphics_command_pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_families.graphics.unwrap())
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let graphics_command_pool = unsafe {
            logical_device
                .create_command_pool(&graphics_command_pool_info, None)
                .expect("A problem with the command pool creation")
        };

        let transfer_command_pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_families.transfer.unwrap())
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let transfer_command_pool = unsafe {
            logical_device
                .create_command_pool(&transfer_command_pool_info, None)
                .expect("A problem with the command pool creation")
        };

        Ok(Pools {
            graphics_command_pool,
            transfer_command_pool
        })
    }

    pub fn cleanup(&self, logical_device: &ash::Device) {
        unsafe {
            logical_device.destroy_command_pool(self.graphics_command_pool, None);
            logical_device.destroy_command_pool(self.transfer_command_pool, None);
        }
    }
}
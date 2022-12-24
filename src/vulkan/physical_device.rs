use ash::vk;

pub struct PhysicalDevice {}

impl PhysicalDevice {
    pub fn pick_physical_device(instance: &ash::Instance) -> Option<(vk::PhysicalDevice, vk::PhysicalDeviceProperties, vk::PhysicalDeviceFeatures)> {
        let physical_devices = unsafe { instance.enumerate_physical_devices().expect("Could not enumerate physical devices!") };

        let mut physical_device: vk::PhysicalDevice = vk::PhysicalDevice::null();
        let mut current_score = 0.0;

        for pd in &physical_devices {
            let score = Self::rate_physical_device(instance, pd);
            if score > current_score {
                current_score = score;
                physical_device = *pd;
            }
        }

        if physical_device == vk::PhysicalDevice::null() { return None; }
        
        let props = unsafe { instance.get_physical_device_properties(physical_device) };
        let features = unsafe { instance.get_physical_device_features(physical_device) };
        let device_name = String::from(
            unsafe { std::ffi::CStr::from_ptr(props.device_name.as_ptr()) }
                .to_str()
                .unwrap()
        );

        let driver_major = props.driver_version >> 22;
        let driver_minor = (props.driver_version >> 12) & 0x3ff;
        let driver_patch = props.driver_version & 0xfff;

        let api_major = vk::api_version_major(props.api_version);
        let api_minor = vk::api_version_minor(props.api_version);
        let api_patch = vk::api_version_patch(props.api_version);
        let api_variant = vk::api_version_variant(props.api_version);

        println!("[Reverie][info] Using {:?} device {} (driver v{}.{}.{} with score {})", 
            props.device_type, device_name, driver_major, driver_minor, driver_patch, current_score);
        println!("[Reverie][info] Device supports Vulkan v{}.{}.{} (variant {}).",
            api_major, api_minor, api_patch, api_variant);
        
        Some((physical_device, props, features))
    }

    pub fn rate_physical_device(instance: &ash::Instance, device: &vk::PhysicalDevice) -> f32 {
        let props = unsafe { instance.get_physical_device_properties(*device) };
        let features = unsafe { instance.get_physical_device_features(*device) };
        let queue_family_properties = unsafe { instance.get_physical_device_queue_family_properties(*device) };

        let mut score = 0.0;

        if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU { score += 1000.0; }
        else if props.device_type == vk::PhysicalDeviceType::VIRTUAL_GPU { score += 500.0; }
        else if props.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU { score += 250.0; }

        // Maximum possible size of textures affects graphics quality
        score += props.limits.max_image_dimension2_d as f32;

        if features.geometry_shader < 1 { // Features are either 0 (not supported) or 1 (supported)
            println!("Device missing geometry shader support, thus your system is not supported!");
            return 0.0;
        }

        let mut found_graphics_queue = false;
        let mut found_transfer_queue = false;
        for (_index, queue_family) in queue_family_properties.iter().enumerate() {
            if queue_family.queue_count > 0 && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) { found_graphics_queue = true; }
            if queue_family.queue_count > 0 && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) { found_transfer_queue = true; }
        }

        if !found_graphics_queue || !found_transfer_queue {
            println!("Physical device missing queues.");
            return 0.0;
        }

        score
    }
}
use ash::vk;

use super::window::VulkanWindow;
use super::debug::VulkanDebug;
use super::physical_device::PhysicalDevice;
use super::queue::*;
use super::logical_device::LogicalDevice;
use super::swapchain::VulkanSwapchain;

const WINDOW_TITLE: &'static str = "Reverie";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct VulkanApp {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub window: VulkanWindow,
    pub debug: VulkanDebug,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub physical_device_features: vk::PhysicalDeviceFeatures,
    pub queue_families: QueueFamilies,
    pub queues: Queues,
    pub device: ash::Device,
    pub swapchain: VulkanSwapchain,
}

impl VulkanApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (event_loop, window) = VulkanWindow::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)?;

        let layer_names = vec!["VK_LAYER_KHRONOS_validation"]; 
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(&entry, &layer_names, &window)
            .expect("Failed to initialize instance!");
        let window = VulkanWindow::new(event_loop, window, &entry, &instance)?;
        
        let debug = VulkanDebug::new(&entry, &instance)?;

        let (physical_device, physical_device_properties, physical_device_features) = PhysicalDevice::pick_physical_device(&instance)
            .expect("No suitable physical device found!");

        let queue_families = QueueFamilies::new(&instance, physical_device, &window)?;

        let (logical_device, queues) = LogicalDevice::new(&instance, physical_device, &queue_families, &layer_names)?;

        let mut swapchain = VulkanSwapchain::new(&instance, physical_device, &logical_device, &window, &queue_families, &queues)?;

        Ok(Self {
            entry,
            instance,
            window,
            debug,
            physical_device,
            physical_device_properties,
            physical_device_features,
            queue_families,
            queues,
            device: logical_device,
            swapchain
        })
    }

    pub fn create_instance(entry: &ash::Entry, layer_names: &[&str], window: &winit::window::Window) -> Result<ash::Instance, vk::Result> {
        let app_name = std::ffi::CString::new("Reverie Engine").unwrap();
        let engine_name = std::ffi::CString::new("Reverie").unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .engine_name(&engine_name)
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::API_VERSION_1_3);

        let layer_names: Vec<std::ffi::CString> = layer_names
            .iter()
            .map(|&ln| std::ffi::CString::new(ln).unwrap())
            .collect();
        
        let layer_name_pointers: Vec<*const i8> = layer_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let mut extension_name_pointers: Vec<*const i8> = 
            vec![
                ash::extensions::ext::DebugUtils::name().as_ptr(),
            ];
        let required_surface_extensions = ash_window::enumerate_required_extensions(&window)
            .unwrap()
            .iter()
            .map(|ext| *ext)
            .collect::<Vec<*const i8>>();
        extension_name_pointers.extend(required_surface_extensions.iter());

        println!("Extensions in use: ");
        for ext in extension_name_pointers.iter() {
            println!("\t{}", unsafe { std::ffi::CStr::from_ptr(*ext).to_str().unwrap() });
        }

        let create_flags = vk::InstanceCreateFlags::default();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layer_name_pointers)
            .enabled_extension_names(&extension_name_pointers)
            .flags(create_flags);

        unsafe { entry.create_instance(&create_info, None) }
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.swapchain.cleanup(&self.device);
            self.device.destroy_device(None);
            self.window.cleanup();
            self.debug.cleanup();
            self.instance.destroy_instance(None)
        };
    }
}
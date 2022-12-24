use ash::vk;

use super::window::RendererWindow;
use super::debug::VulkanDebug;

const WINDOW_TITLE: &'static str = "Reverie";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct VulkanApp {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub window: RendererWindow,
    pub debug: VulkanDebug
}

impl VulkanApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (event_loop, window) = RendererWindow::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)?;

        let layer_names = vec!["VK_LAYER_KHRONOS_validation"]; 
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(&entry, &layer_names, &window).expect("Failed to initialize instance!");
        
        let debug = VulkanDebug::new(&entry, &instance)?;

        let window = RendererWindow::new(event_loop, window, &entry, &instance)?;

        Ok(Self {
            entry,
            instance,
            window,
            debug
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
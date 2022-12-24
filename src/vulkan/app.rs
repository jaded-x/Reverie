use ash::vk;
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc};

use super::window::VulkanWindow;
use super::debug::VulkanDebug;
use super::physical_device::PhysicalDevice;
use super::queue::*;
use super::logical_device::LogicalDevice;
use super::swapchain::VulkanSwapchain;
use super::render_pass::RenderPass;
use super::pipeline::Pipeline;
use super::command_pools::Pools;
use super::vertex_buffer::VertexBuffer;
use super::index_buffer::IndexBuffer;
use super::renderable::Renderable;

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
    pub renderpass: vk::RenderPass,
    pub pipeline: Pipeline,
    pub pools: Pools,
    pub commandbuffers: Vec<vk::CommandBuffer>,
    pub allocator: std::mem::ManuallyDrop<Allocator>,
    pub renderables: Vec<Renderable>
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

        let renderpass = RenderPass::new(&logical_device, physical_device, swapchain.surface_format.format)?;

        swapchain.create_framebuffers(&logical_device, renderpass)?;

        let pipeline = Pipeline::new(&logical_device, &swapchain, &renderpass)?;

        let pools = Pools::new(&logical_device, &queue_families)?;

        let buffer_device_address = false;
        let mut allocator = Allocator::new(&AllocatorCreateDesc {
            instance: instance.clone(),
            device: logical_device.clone(),
            physical_device,
            debug_settings: Default::default(),
            buffer_device_address,
        }).expect("Failed to create allocator!");
        allocator.report_memory_leaks(log::Level::Info);

        let commandbuffers = Self::create_commandbuffers(&logical_device, &pools, swapchain.image_count)?;

        
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
            swapchain,
            renderpass,
            pipeline,
            pools,
            commandbuffers,
            allocator: std::mem::ManuallyDrop::new(allocator),
            renderables: vec![]
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

    pub fn create_commandbuffers(logical_device: &ash::Device, pools: &Pools, amount: usize) -> Result<Vec<vk::CommandBuffer>, vk::Result> {
        let commandbuffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(pools.graphics_command_pool)
            .command_buffer_count(amount as u32);
        
        unsafe { logical_device.allocate_command_buffers(&commandbuffer_allocate_info) }
    }

    pub fn fill_commandbuffers(commandbuffers: &[vk::CommandBuffer], logical_device: &ash::Device, renderpass: &vk::RenderPass, swapchain: &VulkanSwapchain, pipeline: &Pipeline, renderables: &Vec<Renderable>
    ) -> Result<(), vk::Result> {
        unsafe {
            logical_device
                .wait_for_fences(&[swapchain.may_begin_drawing[swapchain.current_image]], true, std::u64::MAX)
                .expect("Fence wait failed!");
        }

        for (i, &commandbuffer) in commandbuffers.iter().enumerate() {
            let commandbuffer_begininfo = vk::CommandBufferBeginInfo::builder();
            unsafe { logical_device.begin_command_buffer(commandbuffer, &commandbuffer_begininfo)?; }
        

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.08, 1.0]
                }
            }];

            let renderpass_begininfo = vk::RenderPassBeginInfo::builder()
                .render_pass(*renderpass)
                .framebuffer(swapchain.framebuffers[i])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x:0, y:0 },
                    extent: swapchain.extent
                })
                .clear_values(&clear_values);

            unsafe {
                logical_device.cmd_begin_render_pass(commandbuffer, &renderpass_begininfo, vk::SubpassContents::INLINE);

                for (_i, renderable) in renderables.iter().enumerate() {
                    logical_device.cmd_bind_pipeline(commandbuffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline);
                    match &renderable.index_buffer {
                        Some(index_buffer) => {
                            logical_device.cmd_bind_index_buffer(commandbuffer, index_buffer.get_buffer(), 0, vk::IndexType::UINT32);

                            for vb in &renderable.vertex_buffers {
                                logical_device.cmd_bind_vertex_buffers(commandbuffer, 0, &[vb.get_buffer()], &[0]);
                                logical_device.cmd_draw_indexed(commandbuffer, index_buffer.get_index_count(), 1, 0, 0, 0);
                            }
                        },
                        None => {
                            for vb in &renderable.vertex_buffers {
                                logical_device.cmd_bind_vertex_buffers(commandbuffer, 0, &[vb.get_buffer()], &[0]);
                                logical_device.cmd_draw(commandbuffer, vb.get_vertex_count(), 1, 0, 0);
                            }
                        }
                    }
                }

                logical_device.cmd_end_render_pass(commandbuffer);
                logical_device.end_command_buffer(commandbuffer)?;
            }
        }
        Ok(())
    }

    pub fn set_window_title(&self, title: &str) {
        self.window.window.set_title(title);
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().expect("Failed to wait for device idle!");

            for renderable in &mut self.renderables {
                renderable.destroy(&self.device, &mut self.allocator);
            }

            self.device.free_command_buffers(self.pools.graphics_command_pool, &self.commandbuffers);

            self.pools.cleanup(&self.device);
            self.pipeline.cleanup(&self.device);
            self.device.destroy_render_pass(self.renderpass, None);
            self.swapchain.cleanup(&self.device);
            self.device.destroy_device(None);
            self.window.cleanup();
            self.debug.cleanup();
            self.instance.destroy_instance(None)
        };
    }
}
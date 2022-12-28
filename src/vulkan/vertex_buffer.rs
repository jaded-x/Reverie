use ash::vk;
use gpu_allocator::vulkan::*;
use gpu_allocator::MemoryLocation;

use super::vertex::Vertex;

pub struct VertexBuffer {
    buffer: vk::Buffer,
    allocation: Allocation,
    vertex_count: u32,
}

impl VertexBuffer {
    pub fn new(device: &ash::Device, allocator: &mut Allocator, size: u64) -> VertexBuffer {
        let vertex_buffer_create_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let vertex_buffer = unsafe {
            device
                .create_buffer(&vertex_buffer_create_info, None)
                .expect("Failed to create Vertex Buffer")
        };

        let mem_requirements = unsafe { device.get_buffer_memory_requirements(vertex_buffer) };
        let location = MemoryLocation::CpuToGpu;

        let allocation = allocator.allocate(&AllocationCreateDesc {
            requirements: mem_requirements,
            location,
            linear: true,
            name: "Vertex Buffer"
        }).expect("Failed to allocaate memory for vertex buffer!");

        unsafe {
            device
                .bind_buffer_memory(vertex_buffer, allocation.memory(), allocation.offset())
                .expect("Failed to bind vertex buffer");
        }

        VertexBuffer {
            buffer: vertex_buffer,
            allocation,
            vertex_count: 0
        }
    }

    pub fn destroy(&mut self, device: &ash::Device, allocator: &mut Allocator) {
        allocator
            .free(std::mem::take(&mut self.allocation))
            .expect("Failed to free vertex buffer memory!");
        unsafe {
            device.destroy_buffer(self.buffer, None);
        }
        drop(self);
    }

    pub fn get_vertex_buffer_size(count: usize) -> u64 {
        (count * std::mem::size_of::<Vertex>()) as u64
    }

    pub fn update_buffer(&mut self, data: &[Vertex]) {
        let dst = self.allocation.mapped_ptr().unwrap().cast().as_ptr();

        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
        }
        self.vertex_count = data.len() as u32;
    }

    pub fn get_buffer(&self) -> vk::Buffer { self.buffer }
    pub fn get_vertex_count(&self) -> u32 { self.vertex_count }
}
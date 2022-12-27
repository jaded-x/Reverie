use ash::vk;
use gpu_allocator::vulkan::Allocator;

use super::vertex_buffer::VertexBuffer;
use super::index_buffer::IndexBuffer;

pub struct Renderable {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer
}

impl Renderable {
    pub fn new(device: &ash::Device, allocator: &mut Allocator, vertex_count: usize, index_count: usize) -> Result<Self, vk::Result> {
        let vertex_buffer = VertexBuffer::new(device, allocator, VertexBuffer::get_vertex_buffer_size(vertex_count));
        let index_buffer = IndexBuffer::new(device, allocator, IndexBuffer::get_index_buffer_size(index_count));

        Ok(Self {
            vertex_buffer,
            index_buffer
        })
    }

    pub fn destroy(&mut self, device: &ash::Device, allocator: &mut Allocator) {
        self.vertex_buffer.destroy(device, allocator);
        self.index_buffer.destroy(device, allocator);
    }
}
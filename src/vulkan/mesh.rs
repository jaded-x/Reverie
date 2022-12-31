use ash::vk;
use gpu_allocator::vulkan::Allocator;

use super::vertex_buffer::VertexBuffer;
use super::index_buffer::IndexBuffer;
use super::vertex::Vertex;

pub struct Mesh {
    pub vertex_buffers: Vec<VertexBuffer>,
    pub index_buffer: Option<IndexBuffer>
}

impl Mesh {
    pub fn new(device: &ash::Device, allocator: &mut Allocator, vertex_count: usize, index_count: usize) -> Result<Self, vk::Result> {
        let mut vertex_buffers = vec![];
        let vertex_buffer = VertexBuffer::new(device, allocator, VertexBuffer::get_vertex_buffer_size(vertex_count));
        vertex_buffers.push(vertex_buffer);
        if index_count > 0 {
            let index_buffer = IndexBuffer::new(device, allocator, IndexBuffer::get_index_buffer_size(index_count));
            Ok(Self {
                vertex_buffers,
                index_buffer: Some(index_buffer)
            })
        } else {
            Ok(Self {
                vertex_buffers,
                index_buffer: None,
            })
        }
    }

    pub fn update_vertex_buffer(&mut self, data: &[Vertex]) {
        self.vertex_buffers[0].update_buffer(data);
    }

    pub fn update_index_buffer(&mut self, data: &[u32]) {
        match self.index_buffer {
            Some(ref mut index_buffer) => {
                index_buffer.update_buffer(data);
            },
            None => {
                println!("No index buffer on mesh");
            }
        }
    }

    pub fn destroy(&mut self, device: &ash::Device, allocator: &mut Allocator) {
        for vertex_buffer in &mut self.vertex_buffers {
            vertex_buffer.destroy(device, allocator);
        }
        if let Some(index_buffer) = &mut self.index_buffer {
            index_buffer.destroy(device, allocator);
        }
    }
}
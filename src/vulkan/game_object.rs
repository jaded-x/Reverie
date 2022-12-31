use std::sync::atomic::{AtomicUsize, Ordering};

use super::mesh::Mesh;

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct GameObject {
    id: usize,
    pub mesh: Mesh,
    pub color: uv::Vec3,
    pub transform2d: Transform2DComponent
}

impl GameObject {
    pub fn new(mesh: Mesh, color: uv::Vec3) -> Self {
        Self {
            id: OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst),
            mesh,
            color,
            transform2d: Transform2DComponent {
                translation: uv::Vec2::default()
            }
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

pub struct Transform2DComponent {
    pub translation: uv::Vec2,
}

impl Transform2DComponent {
    pub fn mat2(&self) -> uv::Mat2 {
        uv::Mat2::identity()
    }
}
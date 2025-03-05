use glm::*;
use wgpu::util::DeviceExt;

#[repr(C)]
pub struct Vertex {
    position: Vec3,
    color: Vec3,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices: [Vertex; 3] = [
        Vertex {position: Vec3::new(-0.75, -0.75, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
        Vertex {position: Vec3::new(0.75, -0.75, 0.0), color: Vec3::new(0.0, 1.0, 0.0)},
        Vertex {position: Vec3::new(0.0, 0.75, 0.0), color: Vec3::new(0.0, 0.0, 1.0)},
    ];
    let bytes: &[u8] = unsafe { any_as_u8_slice(&vertices) };
    let buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX,
    };
    let buffer = device.create_buffer_init(&buffer_descriptor);

    return buffer
}

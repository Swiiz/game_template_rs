use std::marker::PhantomData;

use bytemuck::Pod;
use wgpu::util::DeviceExt;

use crate::engine::graphics::Graphics;

pub mod renderer;
pub mod texture;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct Model<I = u16> {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    _marker: PhantomData<I>,
}

impl<I: Pod> Model<I> {
    pub fn new(ctx: &Graphics, vertices: &[Vertex], indices: &[I]) -> Self {
        let vertex_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertex_buffer,
            index_buffer,
            _marker: PhantomData,
        }
    }

    pub fn cube(ctx: &Graphics, inward_facing: bool) -> Self
    where
        I: From<u8>,
    {
        let positions = [
            // Front face
            ([-0.5, -0.5, 0.5], [0.0, 0.0]),
            ([0.5, -0.5, 0.5], [1.0, 0.0]),
            ([0.5, 0.5, 0.5], [1.0, 1.0]),
            ([-0.5, 0.5, 0.5], [0.0, 1.0]),
            // Back face
            ([0.5, -0.5, -0.5], [0.0, 0.0]),
            ([-0.5, -0.5, -0.5], [1.0, 0.0]),
            ([-0.5, 0.5, -0.5], [1.0, 1.0]),
            ([0.5, 0.5, -0.5], [0.0, 1.0]),
            // Left face
            ([-0.5, -0.5, -0.5], [0.0, 0.0]),
            ([-0.5, -0.5, 0.5], [1.0, 0.0]),
            ([-0.5, 0.5, 0.5], [1.0, 1.0]),
            ([-0.5, 0.5, -0.5], [0.0, 1.0]),
            // Right face
            ([0.5, -0.5, 0.5], [0.0, 0.0]),
            ([0.5, -0.5, -0.5], [1.0, 0.0]),
            ([0.5, 0.5, -0.5], [1.0, 1.0]),
            ([0.5, 0.5, 0.5], [0.0, 1.0]),
            // Top face
            ([-0.5, 0.5, 0.5], [0.0, 0.0]),
            ([0.5, 0.5, 0.5], [1.0, 0.0]),
            ([0.5, 0.5, -0.5], [1.0, 1.0]),
            ([-0.5, 0.5, -0.5], [0.0, 1.0]),
            // Bottom face
            ([-0.5, -0.5, -0.5], [0.0, 0.0]),
            ([0.5, -0.5, -0.5], [1.0, 0.0]),
            ([0.5, -0.5, 0.5], [1.0, 1.0]),
            ([-0.5, -0.5, 0.5], [0.0, 1.0]),
        ];

        let vertices: Vec<Vertex> = positions
            .iter()
            .map(|(pos, uv)| Vertex {
                position: *pos,
                uv: *uv,
            })
            .collect();

        #[rustfmt::skip]
        let mut indices: Vec<I> = vec![
            0 .into(), 1 .into(), 2 .into(), 0 .into(), 2 .into(), 3 .into(), // Front
            4 .into(), 5 .into(), 6 .into(), 4 .into(), 6 .into(), 7 .into(), // Back
            8 .into(), 9 .into(), 10.into(), 8 .into(), 10.into(), 11.into(), // Left
            12.into(), 13.into(), 14.into(), 12.into(), 14.into(), 15.into(), // Right
            16.into(), 17.into(), 18.into(), 16.into(), 18.into(), 19.into(), // Top
            20.into(), 21.into(), 22.into(), 20.into(), 22.into(), 23.into(), // Bottom
        ];

        // Reverse winding order if inward facing
        if inward_facing {
            for tri in indices.chunks_mut(3) {
                tri.swap(1, 2);
            }
        }

        Self::new(ctx, &vertices, &indices)
    }

    pub fn plane(ctx: &Graphics) -> Self
    where
        I: From<u8>,
    {
        let (vertices, indices) = (
            [
                Vertex {
                    position: [-0.5, 0.0, -0.5],
                    uv: [0.0, 1.0],
                },
                Vertex {
                    position: [0.5, 0.0, -0.5],
                    uv: [1.0, 1.0],
                },
                Vertex {
                    position: [0.5, 0.0, 0.5],
                    uv: [1.0, 0.0],
                },
                Vertex {
                    position: [-0.5, 0.0, 0.5],
                    uv: [0.0, 0.0],
                },
            ],
            [0.into(), 1.into(), 2.into(), 0.into(), 2.into(), 3.into()],
        );
        Self::new(ctx, &vertices, &indices)
    }

    pub fn indices_count(&self) -> u32 {
        self.index_buffer.size() as u32 / std::mem::size_of::<u16>() as u32
    }
}

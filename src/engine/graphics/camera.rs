use std::f32::consts::FRAC_PI_2;

use bytemuck::{Pod, Zeroable};
use nalgebra::Point3;
use wgpu::util::DeviceExt;

use crate::engine::{
    graphics::Graphics,
    maths::{Mat4f, Vec2u, Vec3f},
};

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3f,
    pub direction: Vec3f,
    pub up: Vec3f,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let position = Vec3f::new(0.0, 0.0, 5.0);
        let target = Vec3f::new(0.0, 0.0, 0.0);
        let mut camera = Camera {
            position,
            direction: (target - position).normalize(),
            up: Vec3f::new(0.0, 1.0, 0.0),
            yaw: -FRAC_PI_2,
            pitch: 0.0,
            roll: 0.0,
        };
        camera.update_direction_from_angles();
        camera
    }
}

impl Camera {
    pub fn update_direction_from_angles(&mut self) {
        let yaw_rad = self.yaw;
        let pitch_rad = self.pitch;

        let x = pitch_rad.cos() * yaw_rad.cos();
        let y = pitch_rad.sin();
        let z = pitch_rad.cos() * yaw_rad.sin();
        self.direction = Vec3f::new(x, y, z).normalize();

        let world_up = Vec3f::new(0.0, 1.0, 0.0);
        let right = world_up.cross(&self.direction).normalize();
        self.up = self.direction.cross(&right).normalize();
    }

    pub fn get_view_proj_matrices(&self, dims: Vec2u) -> (Mat4f, Mat4f) {
        let aspect_ratio = dims.x as f32 / dims.y as f32;
        let fov_y = FRAC_PI_2;
        let z_near = 0.1;
        let z_far = 100.0;

        let axis = nalgebra::Unit::new_normalize(self.direction);
        let roll_rotation = nalgebra::Rotation3::from_axis_angle(&axis, self.roll);
        let rolled_up = roll_rotation * self.up;

        let view_matrix = Mat4f::look_at_rh(
            &Point3::from(self.position),
            &Point3::from(self.position + self.direction),
            &rolled_up,
        );

        let projection_matrix = Mat4f::new_perspective(aspect_ratio, fov_y, z_near, z_far);
        (view_matrix, projection_matrix)
    }
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct CameraData {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
}

pub struct CameraUniform {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl CameraUniform {
    pub fn new(ctx: &Graphics) -> Self {
        let (view_matrix, proj_matrix) =
            Camera::default().get_view_proj_matrices(ctx.viewport_size);
        let data = CameraData {
            view: view_matrix.into(),
            proj: proj_matrix.into(),
        };
        let camera_uniform_buffer =
            ctx.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Uniform Buffer"),
                    contents: bytemuck::cast_slice(&[data]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let camera_bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX
                            | wgpu::ShaderStages::FRAGMENT
                            | wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("Camera Bind Group Layout"),
                });

        let camera_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        Self {
            bind_group_layout: camera_bind_group_layout,
            uniform_buffer: camera_uniform_buffer,
            bind_group: camera_bind_group,
        }
    }

    pub fn update(&self, ctx: &Graphics, camera: &Camera) {
        let (view_matrix, proj_matrix) = camera.get_view_proj_matrices(ctx.viewport_size);
        let camera_matrices = CameraData {
            view: view_matrix.into(),
            proj: proj_matrix.into(),
        };
        ctx.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_matrices]),
        );
    }
}

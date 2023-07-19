use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use cogrrs::wgpu::TextureFormat;
use cogrrs::TextureRes::FullRes;
use cogrrs::{div_ceil, CoGr, Encoder, Input, Pipeline, ResourceHandle};
use dolly::prelude::{Arm, Position, Smooth, YawPitch};
use dolly::rig::CameraRig;
use glam::{UVec2, Vec3};

use cogrrs::puffin;

use crate::helpers::bool_to_f32;
use crate::key_mapping::{MOVE_BACKWARD, MOVE_DOWN, MOVE_FORWARD, MOVE_LEFT, MOVE_RIGHT, MOVE_UP};

pub struct Camera {
    camera: CameraRig,
    random_seed: u32,
    primary_ray_data: ResourceHandle,
    camera_data: ResourceHandle,
    generate_rays: Pipeline,
    debug_ray_direction: Pipeline,

    pub aperture: f32,
    pub focal_length: f32,
    pub sensor_height: f32,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Zeroable)]
struct CameraGpu {
    position: Vec3,
    aperture: f32,
    direction: Vec3,
    focal_length: f32,
    direction_side: Vec3,
    sensor_height: f32,
    direction_up: Vec3,
    random_seed: u32,
    screen_dimensions: UVec2,
}

pub struct PrimaryRayGenResults {
    pub primary_ray_data: ResourceHandle,
    pub camera_gpu: ResourceHandle,
}

impl Camera {
    pub fn new(gpu: &mut CoGr) -> Self {
        let camera: CameraRig = CameraRig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Position::new(Vec3::Y))
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .build();
        let primary_ray_data = gpu.texture("primary_ray_direction", FullRes, TextureFormat::Rgba32Float);
        let camera_data = gpu.buffer("camera_data", 1, size_of::<CameraGpu>());
        let generate_rays = gpu.pipeline("shaders/generate_rays.hlsl").unwrap();
        let debug_ray_direction = gpu.pipeline("shaders/debug_renders/ray_direction.hlsl").unwrap();
        Self {
            camera,
            random_seed: 1,
            primary_ray_data,
            camera_data,
            generate_rays,
            debug_ray_direction,
            aperture: 2.8,
            focal_length: 1.7,
            sensor_height: 1.57f32,
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input) {
        // all of these can go from -1 to 1
        let move_right = bool_to_f32(input.key_pressed(MOVE_RIGHT)) - bool_to_f32(input.key_pressed(MOVE_LEFT));
        let move_up = bool_to_f32(input.key_pressed(MOVE_UP)) - bool_to_f32(input.key_pressed(MOVE_DOWN));
        let move_forward = bool_to_f32(input.key_pressed(MOVE_FORWARD)) - bool_to_f32(input.key_pressed(MOVE_BACKWARD));

        let move_vec = self.camera.final_transform.rotation * Vec3::new(-move_right, move_up, -move_forward).clamp_length_max(1.0);

        self.camera
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(input.mouse_change()[0], -input.mouse_change()[1]);
        self.camera.driver_mut::<Position>().translate(move_vec * dt * 10.0);
    }

    pub fn generate_rays(&mut self, encoder: &mut Encoder, dt: f32) -> PrimaryRayGenResults {
        puffin::profile_scope!("Generate rays");
        self.random_seed += 1;
        self.camera.update(dt);
        let camera_data = CameraGpu {
            position: self.camera.final_transform.position,
            aperture: self.aperture,
            direction: self.camera.final_transform.forward(),
            focal_length: self.focal_length,
            direction_side: self.camera.final_transform.right(),
            sensor_height: self.sensor_height,
            direction_up: self.camera.final_transform.up(),
            random_seed: self.random_seed,
            screen_dimensions: UVec2::new(encoder.width(), encoder.height()),
        };
        // upload latest camera data to gpu
        encoder.set_buffer_data(&self.camera_data, [camera_data]).unwrap();
        // use latest camera data to calculate new rays
        encoder
            .dispatch_pipeline(
                &mut self.generate_rays,
                (div_ceil(encoder.width(), 32), div_ceil(encoder.height(), 32), 1),
                &[0; 0],
                &[&self.primary_ray_data, &self.camera_data],
            )
            .unwrap();

        PrimaryRayGenResults {
            primary_ray_data: self.primary_ray_data.clone(),
            camera_gpu: self.camera_data.clone(),
        }
    }

    pub fn debug_ray_direction(&mut self, encoder: &mut Encoder, to_screen: &ResourceHandle) {
        encoder
            .dispatch_pipeline(
                &mut self.debug_ray_direction,
                (div_ceil(encoder.width(), 32), div_ceil(encoder.height(), 32), 1),
                &[0; 0],
                &[&self.primary_ray_data, &to_screen],
            )
            .unwrap();
    }
}

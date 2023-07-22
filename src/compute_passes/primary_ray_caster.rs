use cogrrs::wgpu::TextureFormat;
use cogrrs::TextureRes::FullRes;
use cogrrs::{div_ceil, puffin, CoGr, Encoder, Pipeline, ResourceHandle};

use crate::compute_passes::camera::PrimaryRayGenResults;

use super::{ComputePass, ResourceHandles};

pub struct PrimaryRayCaster {
    normal: ResourceHandle,
    depth: ResourceHandle,
    material: ResourceHandle,
    complexity: ResourceHandle,
    trace_ray: Pipeline,
    debug_complexity: Pipeline,
    debug_depth: Pipeline,
    debug_normals: Pipeline,
}

pub struct PrimaryRayCasterResults {
    normal: ResourceHandle,
    depth: ResourceHandle,
    material: ResourceHandle,
    complexity: ResourceHandle,
}

impl ResourceHandles for PrimaryRayCasterResults {}

impl ComputePass for PrimaryRayCaster {
    type Inputs = PrimaryRayGenResults;
    type Outputs = PrimaryRayCasterResults;

    fn new(gpu: &mut CoGr) -> Self {
        let normal = gpu.texture("normal", FullRes, TextureFormat::Rgba8Snorm);
        let depth = gpu.texture("depth", FullRes, TextureFormat::R16Float);
        let material = gpu.texture("material", FullRes, TextureFormat::R8Uint);
        let complexity = gpu.texture("complexity", FullRes, TextureFormat::R16Uint);

        let trace_ray = gpu.pipeline("shaders/trace_primary_rays.hlsl").unwrap();
        let debug_complexity = gpu.pipeline("shaders/debug_renders/complexity.hlsl").unwrap();
        let debug_depth = gpu.pipeline("shaders/debug_renders/depth.hlsl").unwrap();
        let debug_normals = gpu.pipeline("shaders/debug_renders/normals.hlsl").unwrap();
        Self {
            normal,
            depth,
            material,
            complexity,
            trace_ray,
            debug_complexity,
            debug_depth,
            debug_normals,
        }
    }

    fn dispatch(&mut self, encoder: &mut Encoder, inputs: &Self::Inputs) -> PrimaryRayCasterResults {
        // use latest camera data to calculate new rays
        puffin::profile_function!();

        encoder
            .dispatch_pipeline(
                &mut self.trace_ray,
                (div_ceil(encoder.width(), 32), div_ceil(encoder.height(), 32), 1),
                &[0; 0],
                &[
                    &inputs.primary_ray_data,
                    &inputs.camera_gpu,
                    &self.normal,
                    &self.depth,
                    &self.material,
                    &self.complexity,
                ],
            )
            .unwrap();
        PrimaryRayCasterResults {
            normal: self.normal.clone(),
            depth: self.depth.clone(),
            material: self.material.clone(),
            complexity: self.complexity.clone(),
        }
    }

    fn rebuild(&mut self, _gpu: &mut CoGr) {
        todo!()
    }

    fn draw_ui(&mut self, _ui: &mut cogrrs::egui::Ui) {}
}
impl PrimaryRayCaster {
    pub fn debug_complexity(&mut self, encoder: &mut Encoder, to_screen: &ResourceHandle) {
        puffin::profile_function!();
        encoder
            .dispatch_pipeline(
                &mut self.debug_complexity,
                (div_ceil(encoder.width(), 32), div_ceil(encoder.height(), 32), 1),
                &[0; 0],
                &[&self.complexity, to_screen],
            )
            .unwrap();
    }
    pub fn debug_depth(&mut self, encoder: &mut Encoder, to_screen: &ResourceHandle) {
        puffin::profile_function!();
        encoder
            .dispatch_pipeline(
                &mut self.debug_depth,
                (div_ceil(encoder.width(), 32), div_ceil(encoder.height(), 32), 1),
                &[0; 0],
                &[&self.depth, to_screen],
            )
            .unwrap();
    }
    pub fn debug_normals(&mut self, encoder: &mut Encoder, to_screen: &ResourceHandle) {
        puffin::profile_scope!("Debug normals");
        encoder
            .dispatch_pipeline(
                &mut self.debug_normals,
                (div_ceil(encoder.width(), 32), div_ceil(encoder.height(), 32), 1),
                &[0; 0],
                &[&self.normal, to_screen],
            )
            .unwrap();
    }
}

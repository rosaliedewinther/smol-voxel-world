mod camera;
mod primary_ray_caster;

pub use camera::*;
use cogrrs::{egui::Ui, CoGr, Encoder, ResourceHandle};
pub use primary_ray_caster::*;

pub trait ComputePass {
    type Inputs: ResourceHandles;
    type Outputs: ResourceHandles;

    fn new(gpu: &mut CoGr) -> Self;
    fn rebuild(&mut self, gpu: &mut CoGr);
    fn dispatch(&mut self, encoder: &mut Encoder, inputs: &Self::Inputs) -> Self::Outputs;
    fn draw_ui(&mut self, ui: &mut Ui);
}

pub trait ResourceHandles {}
impl ResourceHandles for () {}
impl ResourceHandles for ResourceHandle {}
impl ResourceHandles for [&ResourceHandle] {}
impl<T: ResourceHandles> ResourceHandles for (T, T) {}
impl<T: ResourceHandles> ResourceHandles for (T, T, T) {}
impl<T: ResourceHandles> ResourceHandles for (T, T, T, T) {}
impl<T: ResourceHandles> ResourceHandles for (T, T, T, T, T) {}
impl<T: ResourceHandles> ResourceHandles for (T, T, T, T, T, T) {}
impl<T: ResourceHandles> ResourceHandles for (T, T, T, T, T, T, T) {}
impl<T: ResourceHandles> ResourceHandles for (T, T, T, T, T, T, T, T) {}

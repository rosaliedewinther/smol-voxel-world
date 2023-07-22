use crate::compute_passes::ComputePass;
use crate::smol_voxel_world::TextureFormat::Rgba32Float;
use crate::{compute_passes::Camera, compute_passes::PrimaryRayCaster};
use anyhow::Result;
use cogrrs::wgpu::TextureFormat;
use cogrrs::winit::event::VirtualKeyCode;
use cogrrs::{egui, puffin};
use cogrrs::{CoGr, Game, Input, ResourceHandle, TextureRes::FullRes};

#[derive(Debug, PartialEq)]
enum RenderMode {
    Complexity,
    Depth,
    Normals,
    RayDirection,
}

pub struct SmolVoxelWorld {
    to_screen: ResourceHandle,
    camera: Camera,
    primary_ray_caster: PrimaryRayCaster,
    render_mode: RenderMode,
}

impl Game for SmolVoxelWorld {
    fn on_init(gpu: &mut CoGr) -> Result<Self> {
        let to_screen = gpu.texture("to_screen", FullRes, Rgba32Float);
        let camera = Camera::new(gpu);
        let primary_ray_caster = PrimaryRayCaster::new(gpu);
        Ok(Self {
            to_screen,
            camera,
            primary_ray_caster,
            render_mode: RenderMode::Normals,
        })
    }

    fn on_tick(&mut self, _gpu: &mut CoGr, _dt: f32) -> Result<()> {
        Ok(())
    }

    fn on_render(&mut self, gpu: &mut CoGr, input: &Input, dt: f32) -> Result<()> {
        let mut encoder = gpu.get_encoder_for_draw()?;
        if input.key_pressed(VirtualKeyCode::X) {
            self.camera.update(input, dt);
        }
        let primary_ray_gen_results = self.camera.dispatch(&mut encoder, &());
        let _primary_ray_cast_results = self.primary_ray_caster.dispatch(&mut encoder, &primary_ray_gen_results);

        match self.render_mode {
            RenderMode::Complexity => self.primary_ray_caster.debug_complexity(&mut encoder, &self.to_screen),
            RenderMode::Depth => self.primary_ray_caster.debug_depth(&mut encoder, &self.to_screen),
            RenderMode::Normals => self.primary_ray_caster.debug_normals(&mut encoder, &self.to_screen),
            RenderMode::RayDirection => self.camera.debug_ray_direction(&mut encoder, &self.to_screen),
        }

        encoder.to_screen(&self.to_screen)?;

        encoder.draw_ui(true, true, |ctx| {
            puffin::profile_function!();
            egui::Window::new("debug").show(ctx, |ui| {
                ui.label(format!("fps: {}", 1f32 / dt));
                self.camera.draw_ui(ui);
            });
        })?;
        Ok(())
    }
}

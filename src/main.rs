// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod image_reader;
mod image_view;

use bytes::{Bytes, BytesMut};

use eframe::{egui, epaint::image::ColorImage, CreationContext, HardwareAcceleration};
use egui::{epaint::Vec2, load::SizedTexture, *};

use std::{
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};
pub const WIDTH: usize = 1292;
pub const HEIGHT: usize = 1040;

pub fn main() -> Result<(), eframe::Error> {
    let mut gpu_config = egui_wgpu::WgpuConfiguration::default();
    gpu_config.power_preference = wgpu::PowerPreference::HighPerformance;
    gpu_config.present_mode = wgpu::PresentMode::AutoVsync;
    gpu_config.supported_backends = wgpu::Backends::all();

    let vieport_builder = ViewportBuilder {
        min_inner_size: Some(egui::vec2(400.0, 200.0)),
        inner_size: Some(egui::vec2(800.0, 610.0)),
        window_level: Some(WindowLevel::Normal),
        icon: None,

        minimize_button: Some(true),
        ..Default::default()
    };

    let options = eframe::NativeOptions {
        viewport: vieport_builder,
        hardware_acceleration: HardwareAcceleration::Preferred,
        wgpu_options: gpu_config,
        vsync: true,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "0.29.1 test",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    view_rx: crossbeam_channel::Receiver<Arc<ColorImage>>,
    pub(crate) texture1: Option<TextureHandle>,
    pub(crate) color_image1: Option<Arc<ColorImage>>,
}

impl MyApp {
    pub(crate) fn new(cc: &CreationContext) -> Self {
        let (view_tx, view_rx) = crossbeam_channel::bounded(1);

        let ctx = cc.egui_ctx.clone();

        ctx.options_mut(|options| {
            options.reduce_texture_memory = true;
            options.repaint_on_widget_change = false;
            options.preload_font_glyphs = true;
        });

        let _t1 = thread::spawn(move || image_reader::load_and_pass(view_tx, ctx));

        let mut myapp = MyApp {
            view_rx: view_rx,

            texture1: None,

            color_image1: None,
        };
        myapp
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_texture1(ui);

            self.ui_imageview(ctx, ui);
        });
        if let Ok(color_image) = self.view_rx.try_recv() {
            self.update_image1(color_image);
        }
    }
}

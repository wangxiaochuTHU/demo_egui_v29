use crate::{
    egui, pos2, Arc, ColorImage, ImageData, MyApp, Rect, Sense, SizedTexture, TextureOptions, Vec2,
    HEIGHT, WIDTH,
};

impl MyApp {
    pub fn ui_imageview(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let texture1 = match self.texture1.as_mut() {
            Some(texture) => texture,
            None => return,
        };

        ui.centered_and_justified(|ui| {
            ui.add(
                egui::Image::new(SizedTexture::new(
                    texture1,
                    Vec2 {
                        x: WIDTH as f32,
                        y: HEIGHT as f32,
                    },
                ))
                .uv(Rect {
                    min: pos2(0.0, 0.0),
                    max: pos2(1.0, 1.0),
                })
                .sense(Sense::click_and_drag()),
            );
        });
    }

    #[inline]
    pub(crate) fn update_image1(&mut self, colorimage: Arc<ColorImage>) {
        self.color_image1.replace(colorimage);
    }

    #[inline]
    pub(crate) fn update_texture1(&mut self, ui: &mut egui::Ui) {
        if let Some(colorimage) = self.color_image1.take() {
            match self.texture1.as_mut() {
                Some(texture) => {
                    texture.set(
                        ImageData::Color(colorimage.clone()),
                        TextureOptions::NEAREST,
                    );
                }
                None => {
                    let texture = ui.ctx().load_texture(
                        "1",
                        ImageData::Color(colorimage.clone()),
                        TextureOptions::NEAREST,
                    );

                    self.texture1.replace(texture);
                }
            }
        }
    }
}

use std::time::Duration;

use crate::{egui, sleep, Arc, ColorImage, HEIGHT, WIDTH};
use crossbeam_channel::Sender;

pub fn load_and_pass(view_tx: Sender<Arc<ColorImage>>, ctx: egui::Context) {
    let mut count = 0;
    loop {
        let gray: Vec<u8> = vec![0; WIDTH * HEIGHT];
        let gray_iter = gray.into_iter().enumerate().map(|(k, p)| {
            let row = k % WIDTH;
            let col = k / HEIGHT;

            ((col / 2 + row) % 256 + count) as u8
        });
        let color_image = Arc::new(ColorImage::from_gray_iter([WIDTH, HEIGHT], gray_iter));
        let _ = view_tx.send(color_image);
        ctx.request_repaint();
        sleep(Duration::from_millis(100));
        count += 1;
    }
}

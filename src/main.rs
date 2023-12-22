//! An example of finding lines in a greyscale image.
//! If running from the root directory of this crate you can test on the
//! wrench image in /examples by running
//! `cargo run --example hough ./examples/wrench.jpg ./tmp`

use image::{open, Luma};
use imageproc::definitions::{HasBlack, HasWhite};
use imageproc::map::map_pixels;
use std::path::Path;

fn main() {
    let input_path = Path::new("./res/小弯道2.jpg");
    let output_dir = Path::new("./");

    // 插入图片并灰度化
    let input_image = open(input_path)
        .expect(&format!("无法加载图像 {:?}", input_path))
        .to_luma8();

    for threshold in 0..255 {
        let threshold_img = map_pixels(&input_image, |_, _, p| {
            if p.0[0] > threshold {
                Luma::<u8>::black()
            } else {
                Luma::<u8>::white()
            }
        });
        let threshold_path = output_dir.join(format!("threshold/{}.jpg", threshold));
        threshold_img.save(&threshold_path).unwrap();
    }
}

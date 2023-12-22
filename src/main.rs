//! An example of finding lines in a greyscale image.
//! If running from the root directory of this crate you can test on the
//! wrench image in /examples by running
//! `cargo run --example hough ./examples/wrench.jpg ./tmp`

use image::imageops::{resize, thumbnail};
use image::{open, Luma};
use imageproc::definitions::{HasBlack, HasWhite};
use imageproc::edges::canny;
use imageproc::hough::detect_lines;
use imageproc::hough::draw_polar_lines;
use imageproc::hough::LineDetectionOptions;
use imageproc::hough::PolarLine;
use imageproc::map::{map_pixels, map_pixels_mut};
use std::path::Path;

fn main() {
    let input_path = Path::new("./res/起点.jpg");
    let output_dir = Path::new("./");

    // 插入图片并灰度化 阈值应该为170
    let input_image = open(input_path)
        .expect(&format!("无法加载图像 {:?}", input_path))
        .to_luma8();
    let input_image = thumbnail(&input_image, 10, 10);
    let mut flag = 0 as u8;
    let mut position: Vec<u32> = vec![];
    let threshold_img = map_pixels(&input_image, |x, _, p| {
        if p.0[0] > 170 {
            Luma::<u8>::black()
        } else {
            // flag += 1;
            // if flag == 10 {
            //     position.push(x);
            //     flag = 0;
            // }
            Luma::<u8>::white()
        }
    });
    let threshold_path = output_dir.join("threshold.jpg");
    threshold_img.save(&threshold_path).unwrap();
}

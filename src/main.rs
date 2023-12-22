//! An example of finding lines in a greyscale image.
//! If running from the root directory of this crate you can test on the
//! wrench image in /examples by running
//! `cargo run --example hough ./examples/wrench.jpg ./tmp`

use image::imageops::thumbnail;
use image::{open, Luma};
use imageproc::definitions::{HasBlack, HasWhite};
use imageproc::map::map_pixels;
use std::path::Path;

fn main() {
    let input_path = Path::new("./res/起点.jpg");
    const sharp: u32 = 20;
    // 插入图片并灰度化 阈值应该为170
    let input_image = open(input_path)
        .expect(&format!("无法加载图像 {:?}", input_path))
        .to_luma8();
    let input_image = thumbnail(&input_image, sharp, sharp);
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

    //left
    let res = threshold_img.into_vec();
    for (i, x) in res.iter().enumerate() {
        print!(
            "{}",
            match x {
                255 => "⬛",
                _ => "⬜",
            }
        );
        if i as u32 % sharp == sharp - 1 {
            println!();
        };
    }
    let mut left = 0;
    for i in 0..sharp / 2 {
        for row in 0..sharp {
            if res[(i + row * sharp) as usize] == 255 {
                left += sharp - row;
            }
        }
    }
    let mut right = 0;
    for i in sharp / 2..sharp {
        for row in 0..sharp {
            if res[(i + row * sharp) as usize] == 255 {
                right += sharp - row;
            }
        }
    }
    println!("{}|{}", left, right);
}

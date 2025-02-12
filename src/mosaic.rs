/*
 * MIT License
 *
 * Copyright (c) 2022 Antonio32A (antonio32a.com) <~@antonio32a.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::collections::VecDeque;

use image::imageops::FilterType;
use image::RgbImage;

const SPACING_SIZE: u32 = 10;
const MAX_SIZE: u32 = 4000;

pub fn mosaic(mut images: VecDeque<RgbImage>) -> RgbImage {
    return match images.len() {
        2 => {
            let mut first = images.pop_front().unwrap();
            let mut second = images.pop_front().unwrap();
            let size = calc_horizontal_size(&first, &second);
            let size_mult = calc_multiplier(Size {
                width: size.width,
                height: size.height,
            });

            first = resize_image(
                first,
                Size {
                    width: (size.first_width as f32 * size_mult).round() as u32,
                    height: (size.height as f32 * size_mult).round() as u32,
                },
            );
            second = resize_image(
                second,
                Size {
                    width: (size.second_width as f32 * size_mult).round() as u32,
                    height: (size.height as f32 * size_mult).round() as u32,
                },
            );

            let mut background = create_background(
                (size.width as f32 * size_mult).round() as u32,
                (size.height as f32 * size_mult).round() as u32,
            );
            image::imageops::overlay(
                &mut background, &first,
                0, 0,
            );
            image::imageops::overlay(
                &mut background, &second,
                (first.width() + SPACING_SIZE) as i64, 0,
            );
            background
        }
        3 => {
            let mut first = images.pop_front().unwrap();
            let mut second = images.pop_front().unwrap();
            let mut third = images.pop_front().unwrap();
            let size = calc_horizontal_size(&first, &second);
            let third_size = calc_vertical_size_raw(
                Size { width: size.width, height: size.height },
                Size { width: third.width(), height: third.height() },
            );

            // If the sizing of the 3rd image is weirdly tall then put it to the right of the other 2.
            if third_size.second_height as f32 * 1.5 > size.height as f32 {
                let three_horizontal = calc_horizontal_size_raw(
                    Size { width: size.width, height: size.height },
                    Size { width: third.width(), height: third.height() },
                );
                let first_two_multiplier = three_horizontal.height as f32 / size.height as f32;
                let size_mult = calc_multiplier(Size {
                    width: three_horizontal.width,
                    height: three_horizontal.height,
                });

                first = resize_image(
                    first,
                    Size {
                        width: (size.first_width as f32 * first_two_multiplier * size_mult).round() as u32,
                        height: (three_horizontal.height as f32 * size_mult).round() as u32,
                    },
                );
                second = resize_image(
                    second,
                    Size {
                        width: (size.second_width as f32 * first_two_multiplier * size_mult).round() as u32,
                        height: (three_horizontal.height as f32 * size_mult).round() as u32,
                    },
                );
                third = resize_image(
                    third,
                    Size {
                        width: (three_horizontal.second_width as f32 * size_mult).round() as u32,
                        height: (three_horizontal.height as f32 * size_mult).round() as u32,
                    },
                );

                let mut background = create_background(
                    (three_horizontal.width as f32 * size_mult).round() as u32,
                    (three_horizontal.height as f32 * size_mult).round() as u32,
                );
                image::imageops::overlay(
                    &mut background, &first,
                    0, 0,
                );
                image::imageops::overlay(
                    &mut background, &second,
                    (first.width() + SPACING_SIZE) as i64, 0,
                );
                image::imageops::overlay(
                    &mut background, &third,
                    (first.width() + SPACING_SIZE + second.width() + SPACING_SIZE) as i64, 0,
                );
                background
            } else {
                let height_multiplier = third_size.width as f32 / size.width as f32;
                let size_mult = calc_multiplier(Size {
                    width: third_size.width,
                    height: third_size.height,
                });

                first = resize_image(
                    first,
                    Size {
                        width: (size.first_width as f32 * size_mult).round() as u32,
                        height: (size.height as f32 * height_multiplier * size_mult).round() as u32,
                    },
                );
                second = resize_image(
                    second,
                    Size {
                        width: (size.second_width as f32 * size_mult).round() as u32,
                        height: (size.height as f32 * height_multiplier * size_mult).round() as u32,
                    },
                );
                third = resize_image(
                    third,
                    Size {
                        width: (third_size.width as f32 * size_mult).round() as u32,
                        height: (third_size.second_height as f32 * size_mult).round() as u32,
                    },
                );

                let mut background = create_background(
                    (third_size.width as f32 * size_mult).round() as u32,
                    (third_size.height as f32 * size_mult).round() as u32,
                );
                image::imageops::overlay(
                    &mut background, &first,
                    0, 0,
                );
                image::imageops::overlay(
                    &mut background, &second,
                    (first.width() + SPACING_SIZE) as i64, 0,
                );
                image::imageops::overlay(
                    &mut background, &third,
                    0, (first.height() + SPACING_SIZE) as i64,
                );
                background
            }
        }
        4 => {
            let mut first = images.pop_front().unwrap();
            let mut second = images.pop_front().unwrap();
            let mut third = images.pop_front().unwrap();
            let mut fourth = images.pop_front().unwrap();

            let top = calc_horizontal_size(&first, &second);
            let bottom = calc_horizontal_size(&third, &fourth);
            let all = calc_vertical_size_raw(
                Size { width: top.width, height: top.height },
                Size { width: bottom.width, height: bottom.height },
            );
            let top_width_mult = all.first_height as f32 / top.height as f32;
            let bottom_width_mult = all.second_height as f32 / bottom.height as f32;
            let size_mult = calc_multiplier(Size { width: all.width, height: all.height });

            first = resize_image(first, Size {
                width: (top.first_width as f32 * top_width_mult * size_mult).round() as u32,
                height: (all.first_height as f32 * size_mult).round() as u32,
            });
            second = resize_image(second, Size {
                width: (top.second_width as f32 * top_width_mult * size_mult).round() as u32,
                height: (all.first_height as f32 * size_mult).round() as u32,
            });
            third = resize_image(third, Size {
                width: (bottom.first_width as f32 * bottom_width_mult * size_mult).round() as u32,
                height: (all.second_height as f32 * size_mult) as u32,
            });
            fourth = resize_image(fourth, Size {
                width: (bottom.second_width as f32 * bottom_width_mult * size_mult).round() as u32,
                height: (all.second_height as f32 * size_mult) as u32,
            });

            let mut background = create_background(
                (all.width as f32 * size_mult) as u32,
                (all.height as f32 * size_mult) as u32,
            );

            // We also multiply the spacing by how much the width increased, this isn't ideal but
            // it's barely noticeable and it's how the original FixTweet-Mosaic code works.
            image::imageops::overlay(
                &mut background, &first,
                0, 0,
            );
            image::imageops::overlay(
                &mut background, &second,
                (first.width() as f32 + SPACING_SIZE as f32 * top_width_mult) as i64,
                0,
            );
            image::imageops::overlay(
                &mut background, &third,
                0,
                (first.height() + SPACING_SIZE) as i64,
            );
            image::imageops::overlay(
                &mut background, &fourth,
                (third.width() as f32 + SPACING_SIZE as f32 * bottom_width_mult) as i64,
                (first.height() + SPACING_SIZE) as i64,
            );
            background
        }
        _ => panic!("impossible image length")
    };
}

fn create_background(width: u32, height: u32) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    for pixel in img.enumerate_pixels_mut() {
        *pixel.2 = image::Rgb([0, 0, 0]);
    }
    img
}

fn calc_horizontal_size(first: &RgbImage, second: &RgbImage) -> HorizontalSize {
    calc_horizontal_size_raw(
        Size { width: first.width(), height: first.height() },
        Size { width: second.width(), height: second.height() },
    )
}

fn calc_horizontal_size_raw(first: Size, second: Size) -> HorizontalSize {
    let mut small = second;
    let mut big = first;
    let mut swapped = false;
    if second.height > first.height {
        small = first;
        big = second;
        swapped = true
    }

    let small_width =
        (big.height as f32 / small.height as f32 * small.width as f32).round() as u32;
    HorizontalSize {
        width: small_width + SPACING_SIZE + big.width,
        height: big.height,
        first_width: if swapped { small_width } else { big.width },
        second_width: if swapped { big.width } else { small_width },
    }
}

fn calc_vertical_size_raw(first: Size, second: Size) -> VerticalSize {
    let mut small = second;
    let mut big = first;
    let mut swapped = false;
    if second.width > first.width {
        small = first;
        big = second;
        swapped = true
    }

    let small_height =
        (big.width as f32 / small.width as f32 * small.height as f32).round() as u32;
    VerticalSize {
        width: big.width,
        height: small_height + SPACING_SIZE + big.height,
        first_height: if swapped { small_height } else { big.height },
        second_height: if swapped { big.height } else { small_height },
    }
}

fn calc_multiplier(size: Size) -> f32 {
    let biggest = if size.width > size.height {
        size.width
    } else {
        size.height
    };

    if biggest > MAX_SIZE {
        MAX_SIZE as f32 / biggest as f32
    } else {
        1.0 as f32
    }
}

fn resize_image(image: RgbImage, size: Size) -> RgbImage {
    if image.width() != size.width && image.height() != size.height {
        return image::imageops::resize(
            &image,
            size.width,
            size.height,
            FilterType::Triangle, // The original uses Lanczos3 but in practice the difference is not visible.
        );
    }
    return image;
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub struct HorizontalSize {
    pub width: u32,
    pub height: u32,
    pub first_width: u32,
    pub second_width: u32,
}

pub struct VerticalSize {
    pub width: u32,
    pub height: u32,
    pub first_height: u32,
    pub second_height: u32,
}

use std::fs;
use std::io::Read;
use image::{ ImageBuffer, Rgb, RgbImage };
use rusttype::{ Font, Scale };
use imageproc::drawing::{ draw_text_mut, text_size };

#[allow(dead_code)]
pub fn text_to_image(
    text: &str,
    font_color: Option<Rgb<u8>>,
    background_color: Option<Rgb<u8>>
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = (72, 72);

    let _background_color = background_color.unwrap_or(Rgb([0, 0, 0]));
    let font_color = font_color.unwrap_or(Rgb([255, 255, 255]));

    let mut image = RgbImage::new(width, height);
    // let mut image = ImageBuffer::from_fn(width, height, |_x, _y| background_color);

    let font_path = "assets/fonts/Poxel.ttf";
    let mut font_data = Vec::new();
    fs::File
        ::open(font_path)
        .expect("Error opening font file")
        .read_to_end(&mut font_data)
        .expect("Error reading font file");

    let font = Font::try_from_vec(font_data).expect("Error loading font");

    let font_size = 20.0;
    let scale = Scale::uniform(font_size);
    let vertical_spacing = 2.0;

    // split text into lines by \n
    let lines: Vec<&str> = text.split('\n').collect();
    // Get the height of each line and calculate the total height with spacing
    let line_heights: Vec<i32> = lines
        .iter()
        .map(|line| text_size(scale, &font, line).1 as i32)
        .collect();
    let total_height: i32 =
        line_heights.iter().sum::<i32>() + ((vertical_spacing * ((lines.len() - 1) as f32)) as i32);

    // Get left offset for each line and top offset for the first line
    let mut offsets = Vec::new();
    let mut current_top_offset = ((height as i32) - total_height) / 2;
    for line in &lines {
        let (w, _) = text_size(scale, &font, line);
        let x = ((width as i32) - (w as i32)) / 2;
        offsets.push((x, current_top_offset));
        current_top_offset += (vertical_spacing as i32) + (text_size(scale, &font, line).1 as i32);
    }

    // Draw text lines
    for (i, line) in lines.iter().enumerate() {
        draw_text_mut(&mut image, font_color, offsets[i].0, offsets[i].1, scale, &font, line);
    }

    // draw_text_mut(&mut image, Rgb([0u8, 0u8, 255u8]), 0, 0, scale, &font, text);

    image
}
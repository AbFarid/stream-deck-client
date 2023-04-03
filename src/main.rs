mod text_to_image;

use elgato_streamdeck::{list_devices, new_hidapi, StreamDeck};
// use text_to_image::text_to_image;
use image::{DynamicImage};
use image::io::Reader as ImageReader;
use std::path::Path;

fn main() {
    let hid = new_hidapi().unwrap();

    let (kind, serial) = list_devices(&hid).remove(0);

    // Connect to the device
    let device = StreamDeck::connect(&hid, kind, &serial)
        .expect("Failed to connect");
    
    // Print out some info from the device
    println!(
        "Connected to '{}' with version '{}'",
        device.serial_number().unwrap(),
        device.firmware_version().unwrap()
    );
    
    // Set device brightness
    device.set_brightness(35).unwrap();
    
    // Use image-rs to load an image
    // let image = open("no-place-like-localhost.jpg").unwrap();
    // let text = "Hello\nWorld";
    // let img_buffer = text_to_image(
    //     btn_text,
    //     Some(Rgb([60, 110, 222])),
    //     None
    // );
    // img_buffer.save("output.png").unwrap();
    // let image = DynamicImage::ImageRgb8(img_buffer);
    
    // Write it to the device
    // device.set_button_image(7, image).unwrap();

// Load all the frames into memory
    let mut frames: Vec<DynamicImage> = Vec::new();
    for i in 0..=87 {
        let frame_path = format!("assets/rickroll/frame_{}.jpg", i);
        let img_buffer = ImageReader::open(Path::new(&frame_path))
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to decode image");

        frames.push(DynamicImage::ImageRgb8(img_buffer.to_rgb8()));
    }

    let mut i = 0;
    let mut j = 0;

    loop {
        let image = &frames[i];

        // Set the button image
        device.set_button_image(7, image.clone()).unwrap();

        // Increment the frame counter
        i += 1;

        // Reset the frame counter and increment the loop counter after the last frame
        if i > 87 {
            i = 0;
            j += 1;
        }

        // Break the loop after 3 full loops through the frames
        if j == 3 { break; }

        // Sleep for a short duration between frames
        std::thread::sleep(std::time::Duration::from_millis(65));
    }

    // Clear the button image
    device.clear_button_image(7).unwrap();
}
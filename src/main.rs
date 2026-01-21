use std::env;
use std::path::{Path, PathBuf};
use image::{ImageReader, RgbImage};
use std::error::Error;
use std::fs;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: crypt-files <image.png> <file_to_hide>");
        std::process::exit(1);
    }

    let image_path = &args[1];
    let file_path = &args[2];

    let mut img = get_bytes(image_path)?;
    let data = img.as_mut();

    let file_bytes = fs::read(file_path)?;

    if file_bytes.len() * 8 > data.len() {
        eprintln!("File is too large to hide in this image.");
        std::process::exit(1);
    }

    let mut data_index = 0;
    for byte in file_bytes {
        for bit in byte_to_bits(byte) {
            change_lsb(data, data_index, bit);
            data_index += 1;
        }
    }

    println!("{:08b}", img.as_raw()[0]);
    save_image(&img, image_path)?;
    Ok(())
}

fn get_bytes(image_path: &str) -> Result<RgbImage, Box<dyn Error>> {
    let image = ImageReader::open(image_path)?.decode()?;
    Ok(image.to_rgb8())
}

fn change_lsb(data: &mut [u8], color_channel: usize, value: bool) {
    data[color_channel] = (data[color_channel] & !1) | (value as u8);
}

fn save_image(image: &RgbImage, old_path: &str) -> Result<(), Box<dyn Error>> {
    let old_path = Path::new(old_path);
    let mut new_path = PathBuf::from(old_path);

    if let Some(stem) = old_path.file_stem() {
        let stem_str = stem.to_string_lossy();

        let ext_str = old_path.extension()
            .map(|e| e.to_string_lossy())
            .unwrap_or_else(|| "png".into()); // fallback

        let new_filename = format!("{}-formatted.{}", stem_str, ext_str);

        new_path.set_file_name(new_filename);
    }

    image.save(&new_path)?;
    println!("Saved as {:?}", new_path);
    Ok(())
}

fn byte_to_bits(byte: u8) -> [bool; 8] {
    [
        byte & 0b1000_0000 != 0,
        byte & 0b0100_0000 != 0,
        byte & 0b0010_0000 != 0,
        byte & 0b0001_0000 != 0,
        byte & 0b0000_1000 != 0,
        byte & 0b0000_0100 != 0,
        byte & 0b0000_0010 != 0,
        byte & 0b0000_0001 != 0,
    ]
}

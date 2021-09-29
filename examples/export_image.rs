use image::{ImageBuffer, RgbaImage, Rgba};
use rosti::*;

pub fn save_raster_as_image(filename: &str, raster: GreyscaleRaster){
    
    let (width, height) = raster.get_size();
    let mut imgbuf: RgbaImage = ImageBuffer::new(width as u32, height as u32);

    for x in 0..width {
        for y in 0..height {
            let pix = raster.get_pixel(x as u32, y as u32);
            let i_pix = 255 - pix;
            
            imgbuf.put_pixel(x as u32, y as u32, Rgba([i_pix, i_pix, i_pix, 255]));
        }    
    }

    imgbuf.save(filename).unwrap();
}
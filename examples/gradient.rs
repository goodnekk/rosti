use rosti::*;

mod export_image;

fn main() {
    let size = 256;
    let mut raster = GreyscaleRaster::new(size, size);

    for x in 0..size {
        for y in 0..size {
            //let distance_to_center = ((center - x)*(center - x) + (center - y)*(center - y)).sqrt();
            let value_x = (128.0 * x as f32 / size as f32) as u8;
            let value_y = (128.0 * y as f32 / size as f32) as u8;
            raster.set_pixel(x, y, value_x+value_y);
        }
    }
    
    export_image::save_raster_as_image("test_output/gradient.png", raster);
}
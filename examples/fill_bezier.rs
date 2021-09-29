use rosti::*;

mod export_image;

use std::time::{Duration, Instant};


fn main() {
    let mut raster = GreyscaleRaster::new(800, 800);

    let mut path = Path::new_with_start(20.0, 400.0);
    path.quadratic_curve_to(400.0, 20.0, 780.0, 400.0);
    path.quadratic_curve_to(400.0, 780.0, 20.0, 400.0);
    path.close();

    let now = Instant::now();

    fill_path(path, &mut raster);
    //draw_dda_path(path, &mut raster);
    println!("rasterization took: {} ms", now.elapsed().as_millis());
    export_image::save_raster_as_image("test_output/filled_curves.png", raster);
    
}
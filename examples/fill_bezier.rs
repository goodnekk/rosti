use rosti::*;

mod export_image;

use std::time::{Instant};


fn main() {
    let mut raster = GreyscaleRaster::new(800, 800);

    let mut path = Path::new_with_start(20.0, 400.0);
    path.quadratic_curve_to(400.0, 20.0, 780.0, 400.0);
    path.cubic_curve_to(780.0, 780.0, 20.0, 780.0, 20.0, 400.0);
    path.close();

    let mut total_time = 0;

    for _ in 0..1000 {
        let now = Instant::now();

        fill_path(&path, &mut raster);
        //draw_dda_path(path, &mut raster);
        total_time += now.elapsed().as_nanos();
    }

    let average_time = total_time / 1000;

    println!("rasterization took on average: {} ns", average_time);
    
    export_image::save_raster_as_image("test_output/filled_curves.png", raster);
    
}
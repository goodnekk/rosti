use rosti::*;

mod export_image;


fn main() {
    let mut raster = GreyscaleRaster::new(800, 800);

    let mut path = Path::new_with_start(20.0, 20.0);
    path.cubic_curve_to(200.0, 40.0, 500.0, 700.0, 790.0, 790.0);
    path.quadratic_curve_to(750.0, 20.0, 20.0, 20.0);

    draw_dda_path(path, &mut raster);

    export_image::save_raster_as_image("test_output/curves.png", raster);
    
}
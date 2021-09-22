use rosti::*;

#[test]
fn it_sets_pixels() {
    let size = 30;
    let mut raster = GreyscaleRaster::new(size, size);

    for x in 0..size {
        for y in 0..size {
            //let distance_to_center = ((center - x)*(center - x) + (center - y)*(center - y)).sqrt();
            let value_x = (128.0 * x as f32 / size as f32) as u8;
            let value_y = (128.0 * y as f32 / size as f32) as u8;
            raster.set_pixel(x, y, value_x+value_y);
        }
    }
    
    //raster.set_pixel(29, 29, 255);
    raster.debug_log();
}

#[test]
fn it_draws_lines() {
    let size = 30;
    let mut raster = GreyscaleRaster::new(size, size);

    //let line1 = Line::new(0.0,0.0,29.0,15.0);
    //let line2 = Line::new(0.0,10.0,15.0,29.0);
    let line1 = Line::new(15.0,0.0,15.0,29.0);
    let line2 = Line::new(0.0,15.0,29.0,15.0);
    let line3 = Line::new(7.0,29.0,18.0,0.0);
    let line4 = Line::new(3.0,29.0,29.0,7.0);
    let line5 = Line::new(29.0,22.0,0.0,7.0);

    naive_line(line1, &mut raster);
    naive_line(line2, &mut raster);
    naive_line(line3, &mut raster);
    naive_line(line4, &mut raster);
    naive_line(line5, &mut raster);

    raster.debug_log();
}
pub mod path;
pub use path::*;

pub mod raster;
pub use raster::*;

pub mod fill;
pub use fill::*;

pub fn draw_path_pix(path: Path, raster: &mut GreyscaleRaster) {
    let points = path.as_points();
    
    for p in points {
        raster.set_pixel(p.x.round() as u32, p.y.round() as u32, 255);
    }
}

pub fn draw_dda_path(path: Path, raster: &mut GreyscaleRaster) {
    let points = path.as_points();
    
    for s in points.windows(2) {
        let prev = s[0];
        let next = s[1];
        draw_dda_line(prev, next, raster);
    }
}

pub fn draw_dda_line(a: Point, b: Point, raster: &mut GreyscaleRaster) {
    let rise = b.y - a.y;
    let run = b.x - a.x;
    
    //Get rounded x and ys
    let mut x1 = a.x.round() as u32;
    let mut y1 = a.y.round() as u32;
    let mut x2 = b.x.round() as u32;
    let mut y2 = b.y.round() as u32;

    //Swap around directions
    if x1 > x2 {
        let t = x1;
        x1 = x2;
        x2 = t;
    }

    if y1 > y2 {
        let t = y1;
        y1 = y2;
        y2 = t;
    }

    if run == 0.0 {
        //vertical case
        for y in y1..y2+1 {
            raster.set_pixel(x1, y, 255);
        }
    } else {
        let m = rise / run; //slope
        let b = a.y - m * a.x;//offset
        
        if m<=1.0 && m >= -1.0 {
            //more horizontal than vertical, so find y for each x
            for x in x1..x2+1 {
                let y = (m * x as f32 + b).round();
                raster.set_pixel(x, y as u32, 255);
            }
        } else {
            //more vertical than horizontal, so find x for each y
            for y in y1..y2+1 {
                let x = ((y as f32 - b) / m).round();
                raster.set_pixel(x as u32, y, 255);
            }
        }
    }
}
pub mod geometry;
pub use geometry::*;

pub mod raster;
pub use raster::*;

pub fn naive_line(line: Line, raster: &mut GreyscaleRaster) {
    let rise = (line.y2 - line.y1);
    let run = (line.x2 - line.x1);
    
    //Get rounded x and ys
    let mut x1 = line.x1.round() as usize;
    let mut y1 = line.y1.round() as usize;
    let mut x2 = line.x2.round() as usize;
    let mut y2 = line.y2.round() as usize;

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
        let b = line.y1 - m * line.x1;//offset
        
        if m<=1.0 && m >= -1.0 {
            //more horizontal than vertical, so find y for each x
            for x in x1..x2+1 {
                let y = (m * x as f32 + b).round() as usize;
                raster.set_pixel(x, y, 255);
            }
        } else {
            //more vertical than horizontal, so find x for each y
            for y in y1..y2+1 {
                let x = ((y as f32 - b) / m).round() as usize;
                raster.set_pixel(x, y, 255);
            }
        }

    }
}
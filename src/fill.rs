use crate::path::{Path, Point};
use crate::raster::GreyscaleRaster;

pub fn fill_path(mut path: Path, raster: &mut GreyscaleRaster) {
    let mut vertical_slices = sort_path_into_vertical_slices(path);

    let mut active_vertical_slices = Vec::new();
    //scanline rasterize slices
    
    let (width, height) = raster.get_size();

    let mut row_intersections = Vec::with_capacity(vertical_slices.len());
    let mut to_remove = Vec::new();
    let mut to_insert = Vec::new();

    //Todo: Find top and bottom of figure

    for y in 0..height {
        let float_y = y as f32;
        
        //check if we should add any of the vertical slices
        for i in 0..vertical_slices.len() {
            let top = vertical_slices[i].last().unwrap();
            if top.y < float_y {
                to_insert.push(i);
            }
        }

        for i in to_insert.iter().rev() {
            active_vertical_slices.push(vertical_slices.remove(*i));
        }
        
        for (i, slice) in active_vertical_slices.iter_mut().enumerate() {
            let mut prev_point = slice[slice.len() - 1];
            let mut next_point = slice[slice.len() - 2];

            let x_intersection = prev_point.x + (float_y-prev_point.y) / (next_point.y-prev_point.y) * (next_point.x-prev_point.x); 
            row_intersections.push(x_intersection as usize);
            
            //check if we need to progress next round
            if next_point.y < float_y + 1.0 {
                if slice.len() > 2 {
                    slice.pop();
                } else {
                    to_remove.push(i);
                }
            }
        }

        for i in to_remove.iter().rev() {
            active_vertical_slices.remove(*i);
        }

        row_intersections.sort();
        for i in (0..row_intersections.len()).step_by(2) {
            let left = row_intersections[i];
            let right = row_intersections[i+1];
            for x in left..right {
                raster.set_pixel(x as u32, y as u32, 255);
            }
        }

        row_intersections.clear();
        to_remove.clear();
        to_insert.clear();
    }
}

pub fn sort_path_into_vertical_slices(path: Path) -> Vec<Vec<Point>> {
    let points = path.as_points();

    //Convert to vertical slices
    let mut vertical_slices = Vec::new();
    let mut current_vertical_slice = Vec::new();

    //Check initial direction
    let mut is_moving_down = points[0].y < points[1].y;

    for pts in points.windows(2) {
        let prev = pts[0];
        let next = pts[1];

        //Skip ahead for horizontal sections
        if prev.y == next.y {
            continue;
        }

        if is_moving_down {
            if prev.y < next.y {
                current_vertical_slice.insert(0, prev);
            } else {
                is_moving_down = false;
                vertical_slices.push(current_vertical_slice);
                current_vertical_slice = Vec::new();
            }
        } else {
            if prev.y > next.y {
                current_vertical_slice.push(prev);
            } else {
                is_moving_down = true;
                vertical_slices.push(current_vertical_slice);
                current_vertical_slice = Vec::new();
            }
        }
    }

    //Push last vertical slice
    if vertical_slices.len() % 2 == 0{
        if is_moving_down { 
            vertical_slices[0].extend_from_slice(&current_vertical_slice);
        } else {
            //If the amount of slices is even, append the last slice to the first one
            let mut new_first_slice = current_vertical_slice;
            new_first_slice.extend_from_slice(&vertical_slices[0]);
            vertical_slices[0] = new_first_slice;
        }
    } else {
        vertical_slices.push(current_vertical_slice);
    }

    return vertical_slices

}
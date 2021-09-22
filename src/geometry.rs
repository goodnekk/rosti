pub struct Line {
    pub x1: f32, pub y1: f32, 
    pub x2: f32, pub y2: f32,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line {
            x1, y1,
            x2, y2,
        }
    }
}
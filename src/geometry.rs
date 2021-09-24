#[derive(Debug, Clone)]
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

    //Returns the line with the left-most point as the first one
    pub fn get_left_to_right(&self) -> Line {
        if self.x1 > self.x2 {
            return self.get_reverse();
        }
        return self.clone();
    }

    //Returns the line with the top-most point as the first one
    pub fn get_top_to_bottom(&self) -> Line {
        if self.y1 > self.y2 {
            return self.get_reverse();
        }
        return self.clone();
    }
    
    //Swap the two points
    pub fn get_reverse(&self) -> Line {
        Line {
            x1: self.x2, y1: self.y2,
            x2: self.x1, y2: self.y1
        }
    }
}

pub struct QuadraticBezierCurve {
    pub x1: f32, pub y1: f32, 
    pub x2: f32, pub y2: f32,
    pub x3: f32, pub y3: f32,
    pub x4: f32, pub y4: f32,
}

pub struct PathBuilder {
    first_point: (f32, f32),
    last_point: (f32, f32),
    path: Vec<Line>
}


//Utility Structure for creating spliced polygons & path
impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder {
            first_point: (0.0, 0.0),
            last_point: (0.0, 0.0),
            path: Vec::new()
        }
    }

    pub fn begin(&mut self, x: f32, y: f32) {
        self.first_point = (x, y);
        self.last_point = (x, y);
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.push(Line {
            x1: self.last_point.0, y1: self.last_point.1,
            x2: x, y2: y,
        });

        self.last_point = (x, y);
    }

    pub fn close(&mut self) {
        self.path.push(Line {
            x1: self.last_point.0,  y1: self.last_point.1,
            x2: self.first_point.0, y2: self.first_point.0,
        });
    }

    //This consumes self
    pub fn build(self) -> Path {
        Path {
            segments: self.path
        }
    }
}

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<Line>
}



const SEG: f32 = 0.3; //minimum segment size for curves

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32, 
    pub y: f32
}

impl Point {
    pub fn distance(a: &Point, b: &Point) -> f32 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        return (dx*dx + dy*dy).sqrt()
    }

    //avoid calculating squares and roots
    pub fn fast_dist(a: &Point, b: &Point) -> f32 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        return dx*dx + dy*dy
    }

    pub fn mid_point(a: &Point, b: &Point) -> Point {
        let x = (a.x + b.x) / 2.0;
        let y = (a.y + b.y) / 2.0;

        return Point {x, y}
    }
}

#[derive(Debug)]
pub enum PathOp {
    Line {a: Point},
    QuadraticCurve {a: Point, b: Point},
    CubicCurve {a: Point, b: Point, c: Point},
}

#[derive(Debug)]
pub struct Path {
    start: Point,
    ops: Vec<PathOp>,
    closed: bool
}

impl Path {
    pub fn new() -> Path {
        Path {
            start: Point {x: 0.0, y: 0.0},
            ops: Vec::new(),
            closed: false
        }
    }

    pub fn new_with_start(x: f32, y: f32) -> Path {
        Path {
            start: Point {x, y},
            ops: Vec::new(),
            closed: false
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.start = Point {x, y}
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.ops.push(PathOp::Line {
            a: Point {x, y}
        });
    }

    pub fn quadratic_curve_to(&mut self, xa: f32, ya: f32, xb: f32, yb: f32) {
        self.ops.push(PathOp::QuadraticCurve {
            a: Point {x: xa, y: ya},
            b: Point {x: xb, y: yb}
        });
    }

    pub fn cubic_curve_to(&mut self, xa: f32, ya: f32, xb: f32, yb: f32, xc: f32, yc: f32) {
        self.ops.push(PathOp::CubicCurve {
            a: Point {x: xa, y: ya},
            b: Point {x: xb, y: yb},
            c: Point {x: xc, y: yc}
        });
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn as_points(&self) -> Vec<Point>{
        let mut points = Vec::new();
        points.push(self.start);
        
        for op in self.ops.iter() {
            match op {
                PathOp::Line {a} =>{
                    points.push(*a);
                },
                PathOp::QuadraticCurve {a, b} => {
                    let new_points = get_quadratic_bezier_points_recursive(points.last().unwrap(), a, b);
                    println!("generated points {}", new_points.len());
                    points.extend_from_slice(&new_points);
                },
                PathOp::CubicCurve {a, b, c} => {
                    let new_points = get_cubic_bezier_points_recursive(points.last().unwrap(), a, b, c);
                    println!("generated points {}", new_points.len());
                    points.extend_from_slice(&new_points);
                },
                _=>{}
            }
        }

        //Add the starting point at the end if we're closing 
        if self.closed {
            points.push(self.start);
        }

        return points
    }
}


// Compute Cubic Bezier components
fn get_cubic_bezier_points_recursive(a: &Point, b: &Point, c: &Point, d: &Point) -> Vec<Point>{
    let d1 = Point::distance(a, b);
    let d2 = Point::distance(b, c);
    let d3 = Point::distance(c, d);
    let d4 = Point::distance(a, d);

    //println!("cmp {}+{}+{}={} {}", d1, d2, d3, d1+d2+d3, d4);
    //flatness test
    //fast factor = 0.3345
    if (d1+d2+d3) < 1.001*d4 {
        return vec![d.clone()];
    } else {
        let q0 = Point::mid_point(a, b);
        let q1 = Point::mid_point(b, c);
        let q2 = Point::mid_point(c, d);
        let r0 = Point::mid_point(&q0, &q1);
        let r1 = Point::mid_point(&q1, &q2);
        
        let mut pa = get_cubic_bezier_points_recursive(a, &q0, &r0, &Point::mid_point(&r0, &r1));
        let pb = get_cubic_bezier_points_recursive(&Point::mid_point(&r0, &r1), &r1, &q2, d);
        pa.extend_from_slice(&pb);
        return pa;
    }
}

fn get_cubic_bezier_points(a: &Point, b: &Point, c: &Point, d: &Point, sub_div: usize) -> Vec<Point>{
    let mut points = Vec::new();

    let step_size = 1.0 / sub_div as f32;
    
    let mut last_point = a.clone();
    
    let mut step = step_size;

    for _ in 0..sub_div {
        let new_point = get_cubic_bezier_point_at_offset(a, b, c, d, step);
        last_point = new_point.clone();
        points.push(new_point);
        step += step_size;
    }

    return points
}

fn get_cubic_bezier_point_at_offset(a: &Point, b: &Point, c: &Point, d: &Point, t: f32) -> Point {
    let mt = 1.0 - t;
        
    let f1 = mt*mt*mt;
    let f2 = 3.0*mt*mt*t;
    let f3 = 3.0*mt*t*t;
    let f4 = t*t*t;

    let x = f1*a.x + f2*b.x + f3*c.x + f4*d.x;
    let y = f1*a.y + f2*b.y + f3*c.y + f4*d.y;

    Point {x, y}
}

// Compute Quadratic Bezier components
fn get_quadratic_bezier_points_recursive(a: &Point, b: &Point, c: &Point) -> Vec<Point>{
    let d1 = Point::distance(a, b);
    let d2 = Point::distance(b, c);
    let d3 = Point::distance(a, c);

    //flatness test
    if (d1+d2) < 1.001*d3 {
        return vec![c.clone()];
    } else {
        let q0 = Point::mid_point(a, b);
        let q1 = Point::mid_point(b, c);
        let r0 = Point::mid_point(&q0, &q1);
        
        let mut pa = get_quadratic_bezier_points_recursive(a, &q0, &r0);
        let pb = get_quadratic_bezier_points_recursive(&r0, &q1, c);
        pa.extend_from_slice(&pb);
        return pa;
    }
}

fn get_quadratic_bezier_points(a: &Point, b: &Point, c: &Point, sub_div: usize) -> Vec<Point>{
    let mut points = Vec::new();

    let step_size = 1.0 / sub_div as f32;
    
    let mut last_point = a.clone();
    
    let mut step = step_size;

    for _ in 0..sub_div {
        let new_point = get_cubic_quadratic_point_at_offset(a, b, c, step);
        last_point = new_point.clone();
        points.push(new_point);
        step += step_size;
    }

    return points
}

fn get_cubic_quadratic_point_at_offset(a: &Point, b: &Point, c: &Point, t: f32) -> Point {
    let mt = 1.0 - t;
        
    let f1 = mt*mt;
    let f2 = 2.0*mt*t;
    let f3 = t*t;

    let x = f1*a.x + f2*b.x + f3*c.x;
    let y = f1*a.y + f2*b.y + f3*c.y;

    Point {x, y}
}
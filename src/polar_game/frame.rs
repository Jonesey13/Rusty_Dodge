use polar_game::object::{Part,Point};

#[derive(Clone)]
pub struct PolarFrame{
    parts: Vec<Part>,
}

impl PolarFrame{

    pub fn get_render_parts(&self) -> Vec<Part>{
        self.parts.clone()
    }

    pub fn new(radial_size: u64, angle_size: u64, width: Point, radial_total: f64) -> PolarFrame{
        let mut parts: Vec<Part> = Vec::new();
        let radial_step = radial_total / radial_size as f64;
        let angle_step = 1.0 / angle_size as f64;
        let color = [0.1, 0.1, 1.0, 1.0];
        for r in 0..radial_size{
            for a in 0..angle_size{
                let radial = Point{x: radial_step * r as f64,
                                   y: radial_step * r as f64 + width.y};
                let angle = Point{x: angle_step * a as f64 - width.x,
                                  y: angle_step * (a + 1) as f64 + width.x};
                let part = Part{radial: radial,
                                angle: angle,
                                color: color};
                parts.push(part);
            }
        }

        for r in 0..radial_size{
            for a in 0..angle_size{
                let radial = Point{x: radial_step * r as f64 - width.y,
                                   y: radial_step * (r + 1) as f64 + width.y,};
                let angle = Point{x: angle_step * a as f64,
                                  y: angle_step * a as f64 + width.x};
                let part = Part{radial: radial,
                                angle: angle,
                                color: color};
                parts.push(part);
            }
        }
        PolarFrame{parts: parts}
    }
}

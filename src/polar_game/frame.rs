/*
Handles the Polar Frame
*/

use polar_game::object::{Part,Point};

#[derive(Clone)]
pub struct PolarFrame{
    parts: Vec<Part>,
}

impl PolarFrame{

    pub fn get_render_parts(&self) -> Vec<Part>{
        self.parts.clone()
    }

    pub fn new(radial_spacing: f64, angle_spacing: f64, width: Point, radial_max: f64) -> PolarFrame{
        let mut parts: Vec<Part> = Vec::new();
        let radial_total = (radial_max / radial_spacing) as u64;
        let angle_total: u64 = (1.0 / angle_spacing) as u64;
        let color = [0.1, 0.1, 1.0, 1.0];
        for r in 0..radial_total{
            for a in 0..angle_total{
                let radial = Point{x: radial_spacing * r as f64,
                                   y: radial_spacing * r as f64 + width.y};
                let angle = Point{x: angle_spacing * a as f64 - width.x,
                                  y: angle_spacing * (a + 1) as f64 + width.x};
                let part = Part{radial: radial,
                                angle: angle,
                                color: color};
                parts.push(part);
            }
        }

        for r in 0..radial_total{
            for a in 0..angle_total{
                let radial = Point{x: radial_spacing * r as f64 - width.y,
                                   y: radial_spacing * (r + 1) as f64 + width.y,};
                let angle = Point{x: angle_spacing * a as f64,
                                  y: angle_spacing * a as f64 + width.x};
                let part = Part{radial: radial,
                                angle: angle,
                                color: color};
                parts.push(part);
            }
        }
        PolarFrame{parts: parts}
    }
}

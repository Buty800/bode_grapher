use crate::bound::Bound;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Bode {
    Pole(f64),
    Zero(f64),
}

impl Bode {
    pub fn frequency(&self) -> f64 {
       match self {
            Bode::Pole(p) => *p,
            Bode::Zero(z) => *z,
        } 
    }

    fn slope_change(&self) -> f64 {
       match self {
            Bode::Pole(_) => -20.0,
            Bode::Zero(_) => 20.0,
        } 
    }
}


pub fn points(zeros : &[f64], poles : &[f64]) -> Vec<(f64,f64)> { //~O(n*log(n))
    
    let mut break_points : Vec<Bode> = 
            zeros.iter()
            .map(|v| {Bode::Zero(*v)})
            .chain(
            poles.iter()
            .map(|v| {Bode::Pole(*v)}))
            .collect(); 
    
    break_points.sort_by(|a, b| {
            a.frequency()
            .partial_cmp(&b.frequency())
            .unwrap_or(Ordering::Equal)
    });

    let mut plot_points = Vec::new();
    let mut current_slope = 0.0;
    let mut last_point = (0.0, 0.0);
    plot_points.push(last_point);

    for bp in break_points {
        let x = bp.frequency().log10();
        let y = last_point.1 + (x - last_point.0) * current_slope;
        let new_point = (x, y);

        plot_points.push(new_point);

        last_point = new_point;

        current_slope += bp.slope_change();
    }

    let x = last_point.0 + 1.0;
    let y = last_point.1 + (x - last_point.0) * current_slope;
    plot_points.push((x, y));

    plot_points.dedup();

    plot_points    
}

// req: points is orderd
pub fn bounds(points: &[(f64, f64)]) -> (Bound,Bound) {
    
    let max_x = points.last().unwrap_or(&(0.0,0.0)).0;
    let min_y = points.iter().map(|(_, y)| *y).fold(f64::MAX, f64::min).min(0.0);
    let max_y = points.iter().map(|(_, y)| *y).fold(f64::MIN, f64::max).max(20.0);

    (Bound::new(0.0, max_x, 1.0), Bound::new(min_y, max_y, 20.0))
}

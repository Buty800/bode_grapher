pub struct Bound {
    pub start : f64, 
    pub end : f64,
    step : f64 
}

impl Bound {

    pub fn new(start : f64, end: f64, step : f64) -> Self{
        Self { start, end, step }
    }

    pub fn bound(&self) -> [f64;2]{
        [self.start as f64, self.end as f64]
    }

    pub fn range_closed(&self) -> impl Iterator<Item = f64> {
        let n = ((self.end - self.start) / self.step).floor() as usize;  
        (0..=n).map(|k| self.start + k as f64 * self.step)
    }

    pub fn range_open(&self) -> impl Iterator<Item = f64> {
        let n = ((self.end - self.start) / self.step).floor() as usize;  
        (1..n).map(|k| self.start + k as f64 * self.step)
    }
    
    pub fn labels(&self) -> Vec<String>{    
        self.range_closed().map(|x| x.to_string()).collect()
    } 


}
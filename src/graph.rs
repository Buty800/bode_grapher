use crate::bode;
use crate::bound::{Bound};
use std::env;

use ratatui::{
    widgets::{Axis, Chart, Dataset, GraphType},
    symbols::{self},
    style::{Style, Stylize, Color},
    Frame,
};

pub struct GridGraph {
    points  : Vec<(f64,f64)>, 
    x_bound : Bound,
    y_bound : Bound
}

impl GridGraph {
    
    pub fn from_args() -> Self {

        let args: Vec<String> = env::args().skip(1).collect();
        let separator_index = args.iter().position(|a| a == "/").unwrap();
        let zeros : Vec<f64> = args[..separator_index].iter().map(|n| { n.parse().unwrap() } ).collect();
        let poles : Vec<f64> = args[separator_index+1..].iter().map(|n| { n.parse().unwrap() }).collect();
        let points = bode::points(&zeros, &poles);
        let (x_bound, y_bound) = bode::bounds(&points);
        
        Self {
            points, 
            x_bound,
            y_bound,
        }

    }

    pub fn render(self, frame: &mut Frame){
        
        
        let x_axis = Axis::default()
        .title("\u{03c9} (rad/seg)")
        .style(Style::default().gray())
        .bounds(self.x_bound.bound())
        .labels(self.x_bound.labels());
        
        let y_axis = Axis::default()
                .title("Av (db)")
                .style(Style::default().gray())
                .bounds(self.y_bound.bound())
                .labels(self.y_bound.labels());
        
        
        let v_lines = 
            self.x_bound
            .range_open()
            .map(|x| [(x,self.y_bound.start),(x,self.y_bound.end)]);

        let h_lines = 
            self.y_bound
            .range_open()
            .map(|y| [(self.x_bound.start,y),(self.x_bound.end,y)]);


        let grid_lines : Vec<[(f64,f64);2]> = 
            v_lines
            .chain(h_lines)
            .collect();

        let grid_lines =  grid_lines.iter()
            .map(|p| 
                Dataset::default()
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .data(p)
            )
            .collect();
        
        let line = vec![Dataset::default()
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Yellow))
                .data(&self.points)];
            
        let datasets = [grid_lines,line].concat();

        let chart = Chart::new(datasets)
            .x_axis(x_axis)
            .y_axis(y_axis);
        
        frame.render_widget(chart, frame.area());

    }

}
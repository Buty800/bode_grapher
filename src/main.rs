mod graph;
mod bound;
mod bode;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event},
    DefaultTerminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}
fn run(mut terminal: DefaultTerminal) -> Result<()> {

    let bode = graph::GridGraph::from_args();

    terminal.draw(|frame| bode.render(frame))?;
        loop {
            if let Event::Key(_) = event::read()? {
                return Ok(());
            }
        }
    }


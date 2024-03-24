use ratatui::prelude::*;
use spike_widget_macro::Widget;

mod errors;
mod tui;

#[derive(Widget)]
pub struct Recipe {
    pub name: String,
    pub description: String,
}

pub fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;
    let (mut terminal, _guard) = tui::init()?;
    terminal.insert_before(2, |buf| {
        let recipe = Recipe {
            name: "Pancakes".to_string(),
            description: "Delicious pancakes".to_string(),
        };
        recipe.render(buf.area, buf);
    })?;

    Ok(())
}

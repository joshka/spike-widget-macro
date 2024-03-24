use ratatui::prelude::*;
use spike_widget_macro::Widget;

#[derive(Widget)]
pub struct Recipe {
    pub name: String,
    pub description: String,
}

pub fn main() {
    let recipe = Recipe {
        name: "Pancakes".to_string(),
        description: "Delicious pancakes".to_string(),
    };
    let area = Rect::new(0, 0, 40, 3);
    let mut buffer = Buffer::empty(area);
    recipe.render(area, &mut buffer);
    println!("{:?}", buffer);
}

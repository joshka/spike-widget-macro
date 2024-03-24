# Derive Widget Macro

A small PoC for deriving Ratatui `Widget` trait from a struct.

Given the following type:

```rust
#[derive(Widget)]
pub struct Recipe {
    pub name: String,
    pub description: String,
}
```

and the following rendering code:

```rust
let recipe = Recipe {
    name: "Pancakes".to_string(),
    description: "Delicious pancakes".to_string(),
};
recipe.render(buf.area, buf);
```

This outputs:

```plain
       name: Pancakes
description: Delicious pancakes
```

The macro expands to:

```rust
pub struct Recipe {
    pub name: String,
    pub description: String,
}
impl Widget for &Recipe {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        let row = Rect::new(area.x, y, area.width, area.height - y);
        self.render_name(row, buf);
        y += 1;
        let row = Rect::new(area.x, y, area.width, area.height - y);
        self.render_description(row, buf);
        y += 1;
    }
}
impl Recipe {
    fn render_name(&self, area: Rect, buf: &mut Buffer) {
        const LABEL_WIDTH: u16 = 13u16;
        let label_area = Rect::new(area.x, area.y, LABEL_WIDTH, 1);
        let value_area = Rect::new(
            area.x + LABEL_WIDTH,
            area.y,
            area.width - LABEL_WIDTH,
            1,
        );
        ratatui::text::Text::raw("       name: ").render(label_area, buf);
        ratatui::text::Text::raw(self.name.to_string()).render(value_area, buf);
    }
    fn render_description(&self, area: Rect, buf: &mut Buffer) {
        const LABEL_WIDTH: u16 = 13u16;
        let label_area = Rect::new(area.x, area.y, LABEL_WIDTH, 1);
        let value_area = Rect::new(
            area.x + LABEL_WIDTH,
            area.y,
            area.width - LABEL_WIDTH,
            1,
        );
        ratatui::text::Text::raw("description: ").render(label_area, buf);
        ratatui::text::Text::raw(self.description.to_string()).render(value_area, buf);
    }
}
```

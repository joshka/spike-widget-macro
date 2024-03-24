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

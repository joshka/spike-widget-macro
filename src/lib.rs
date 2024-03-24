use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self, parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    match input.data {
        Data::Struct(data) => {
            let string_fields = data.fields.into_iter().filter(|field| {
                matches!(&field.ty, syn::Type::Path(path) if path.path.segments[0].ident == "String")
            }).collect_vec();

            // get the length of the longest field name so we can align the output
            let label_length = string_fields.iter().map(label_width).max().unwrap_or(0);

            let functions = string_fields
                .iter()
                .map(|field| render_field(field.ident.as_ref().unwrap(), label_length));

            let function_calls = string_fields.iter().map(|field| {
                let function_name = format_ident!("render_{}", field.ident.as_ref().unwrap());
                quote! {
                    let row = Rect::new(area.x, y, area.width, area.height - y);
                    self.#function_name(row, buf);
                    y += 1;
                }
            });
            let expanded = quote! {
                impl Widget for &#name {
                    fn render(self, area: Rect, buf: &mut Buffer) {
                        let mut y = area.y;
                        #(#function_calls)*
                    }
                }

                impl #name {
                    #(#functions)*
                }
            };
            TokenStream::from(expanded)
        }
        _ => panic!("Widget derive only works on structs"),
    }
}

fn label_width(field: &syn::Field) -> usize {
    field.ident.as_ref().unwrap().to_string().len()
}

/// Create a function to render a specific field
fn render_field(name: &syn::Ident, label_length: usize) -> proc_macro2::TokenStream {
    let function_name = format_ident!("render_{}", name);
    let label = format!("{:>label_length$}: ", name);
    let label_length = label.len() as u16;
    quote! {
        fn #function_name(&self, area: Rect, buf: &mut Buffer) {
            const LABEL_WIDTH: u16 = #label_length;
            let label_area = Rect::new(area.x, area.y, LABEL_WIDTH, 1);
            let value_area = Rect::new(area.x + LABEL_WIDTH, area.y, area.width - LABEL_WIDTH, 1);
            ratatui::text::Text::raw(#label).render(label_area, buf);
            ratatui::text::Text::raw(self.#name.to_string()).render(value_area, buf);
        }
    }
}

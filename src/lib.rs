use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    match input.data {
        Data::Struct(data) => {
            // get the length of the longest field name so we can align the output
            let max_label_length = data
                .fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap().to_string().len())
                .max()
                .unwrap_or(0);
            let fields = data.fields.iter().map(|field| {
                let name = &field.ident.as_ref().expect("struct fields must have names");
                // obviously replace this with actual rendering logic
                match &field.ty {
                    syn::Type::Path(path)
                        if path.path.segments.last().unwrap().ident == "String" =>
                    {
                        render_string_field(name, max_label_length)
                    }
                    _ => panic!("Widget derive only works on structs with string fields"),
                }
            });
            let expanded = quote! {
                impl Widget for &#name {
                    fn render(self, area: Rect, buf: &mut Buffer) {
                        let mut y = area.y;
                        #(#fields)*
                    }
                }
            };
            TokenStream::from(expanded)
        }
        _ => panic!("Widget derive only works on structs"),
    }
}

fn render_string_field(name: &syn::Ident, max_label_length: usize) -> proc_macro2::TokenStream {
    quote! {
        let label = format!("{:>label_length$}:", stringify!(#name), label_length = #max_label_length);
        let label = ratatui::text::Text::raw(label);
        let value = ratatui::text::Text::raw(self.#name.to_string());
        label.render(Rect { y, ..area }, buf);
        value.render(Rect { y, x: area.x + #max_label_length as u16 + 2, ..area }, buf);
        y += 1;
    }
}

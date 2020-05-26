use proc_macro2::{Ident, Span};

pub fn ident(ident: &str) -> Ident {
    let mut ident = match syn::parse_str::<Ident>(ident) {
        Ok(ident) => ident,
        Err(_) => syn::parse_str::<Ident>(&format!("r#{}", ident)).unwrap(),
    };
    ident.set_span(Span::call_site());
    ident
}

extern crate proc_macro;
use proc_macro2::{TokenStream, Span};
use proc_quote::{quote, quote_spanned};
use syn::Ident;

#[proc_macro]
pub fn supermod(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path_string = if let proc_macro::TokenTree::Literal(lit) = item.into_iter().next().unwrap() { lit.to_string() } else { panic!() };
	// strip "
	let path = &path_string[1..path_string.len() - 1];
	// get just the module name
	let module_name = Ident::new(&path[path.find("/").map(|x| x + 1).unwrap_or(0)..], Span::call_site());
	let files = std::fs::read_dir(path).unwrap().map(|x| {
		let path = x.unwrap().path();
		let name = path.file_stem().unwrap().to_string_lossy();
		let ident = Ident::new(&name, Span::call_site());
		quote!(pub mod #ident;)
	});

	// gotta do this weird way if you want to use <module>.rs instead of mod.rs inside <module>
	proc_macro::TokenStream::from(quote!(
		mod #module_name { #(#files)* }
		pub use #module_name::*;
	))
}

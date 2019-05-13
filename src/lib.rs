extern crate proc_macro;
use proc_macro2::Span;
use proc_quote::quote;
use syn::Ident;

#[proc_macro]
pub fn supermod(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path_string = if let proc_macro::TokenTree::Literal(lit) = item.into_iter().next().unwrap() { lit.to_string() } else { panic!() };
	// strip "
	let path = &path_string[1..path_string.len() - 1];
	// get just the module name
	let module_name = Ident::new(&path[path.find("/").map(|x| x + 1).unwrap_or(0)..], Span::call_site());
	// turn all files in the subfolder into submodule names
	let idents = std::fs::read_dir(path).unwrap().map(|x| {
		let path = x.unwrap().path();
		let name = path.file_stem().unwrap().to_string_lossy();
		Ident::new(&name, Span::call_site())
	}).collect::<Vec<_>>();

	// gotta do this weird way if you want to use <module>.rs instead of mod.rs inside <module>
	proc_macro::TokenStream::from(quote!(
		mod #module_name { #(pub mod #idents;)* }
		pub use #module_name::{
			#(#idents,)*
		};
	))
}

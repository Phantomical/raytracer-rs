
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::iter::Iterator;

use proc_macro::TokenStream;
use syn::{Ident, VariantData, Ty};

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input(&input.to_string()).unwrap();

	let idents: Vec<(Ty, Ident)> = match ast.body {  
	    syn::Body::Struct(vdata) => {
	        match vdata {
	            VariantData::Struct(fields) => {
	                let mut idents = Vec::new();
	                for ref field in fields.iter() {
	                    match &field.ident {
	                        &Some(ref ident) => idents.push(
								(field.ty.clone(), ident.clone())),
	                        &None => panic!("Your struct is missing a field identity!"),
	                    };
						match &field.
	                }
	            },
	            VariantData::Tuple(_) | VariantData::Unit => {
	                panic!("You can only derive this for normal structs!");
	            },
	        }
	    },
	    syn::Body::Enum(_) => panic!("You can only derive this on structs!"),
	};

	let keys = idents.iter()
		.map(|(ty, ref ident)| String::from(ident.as_ref()))
		.collect();
	let types = idents.iter()
		.map(|(ty, ref ident)| ty)
		.collect();

	let name = &ast.ident;
	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

	let quotes : Vec<quote::Tokens> = Vec::new();

	for (i, key) in keys.enumerate() {
		let ty = types[i];
		let ident = idents[i];

		quotes.push(quote! {
			#ident: {
				let des = Deserializer<#ty>
			}
		});
	}

	let tokens = quote! {
		impl #impl_generics Deserialize for #name #ty_generics #where_clause {
			fn deserialize<'de>(val : &'de Value) -> Self {
				match val {
					Object(map) => Self {
						#(
						#idents: {
							let value = #ty::deserialize(map[#keys]);
							if let Err(e) = value { return Err(e) }
							value.ok().unwrap()
						}
						)*
					_ => Err(DeserializerError::WrongType(val))
				}
			}
		}
	};

	
	
}


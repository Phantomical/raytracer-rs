
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::iter::Iterator;

use proc_macro::TokenStream;
use syn::{VariantData};

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input(&input.to_string()).unwrap();

	let (idents, types) = match ast.body {  
	    syn::Body::Struct(vdata) => {
	        match vdata {
	            VariantData::Struct(fields) => {
	                let mut idents = Vec::new();
					let mut types = Vec::new();
	                for ref field in fields.iter() {
	                    match &field.ident {
	                        &Some(ref ident) => idents.push(ident.clone()),
	                        &None => panic!("Your struct is missing a field identity!"),
	                    };
						types.push(field.ty.clone());
	                }
					(idents, types)
	            },
	            VariantData::Tuple(_) | VariantData::Unit => {
	                panic!("You can only derive this for normal structs!");
	            },
	        }
	    },
	    syn::Body::Enum(_) => panic!("You can only derive this on structs!"),
	};

	let keys : Vec<String> = idents.iter()
		.map(|ident| String::from(ident.as_ref()))
		.collect();
	let types2 = types.clone();

	let name = &ast.ident;
	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
	
	let tokens = quote! {
		impl #impl_generics ::serialization::Deserialize for #name #ty_generics #where_clause {
			fn deserialize<'de>(val : &'de ::serde_json::Value) -> Result<Self, ::serialization::DeserializeError> {
				match *val {
					::serde_json::Value::Object(ref _map) => Ok(Self {
						#(
						#idents: {
							let value = <#types as ::serialization::Deserializer<#types2>>::deserialize(&_map[#keys]);
							if let Err(e) = value { return Err(e) }
							value.ok().unwrap()
						},
						)*
					}),
					_ => Err(::serialization::DeserializeError::WrongType(val))
				}
			}
		}
	};

	tokens.parse().unwrap()
}


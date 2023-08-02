extern crate proc_macro;
use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemStruct, Type};

fn type_to_size(ty: &syn::Type) -> Result<usize, syn::Error> {
    match type_to_string(ty)?.as_str() {
        "bool" => Ok(1),
        "i8" | "u8" => Ok(8),
        "i16" | "u16" => Ok(16),
        "i32" | "u32" | "f32" | "char" => Ok(32),
        "i64" | "u64" | "f64" => Ok(64),
        "i128" | "u128" => Ok(128),
        "usize" => Ok(std::mem::size_of::<usize>()),
        "isize" => Ok(std::mem::size_of::<isize>()),
        _ => Err(syn::Error::new(ty.span(), "Pack_struct macro only support Rusts primitive types!\nThese are: bool, i8, u8, i16, u16, i32, u32, f32, char, i64, u64, f64, i128, u128, usize, isize.")),
    }
}

fn type_to_string(ty: &syn::Type) -> Result<String, syn::Error> {
    match ty {
        Type::Path(typepath) if typepath.qself.is_none() => {
            let type_ident = &typepath.path.segments.last().ok_or_else(|| {
                syn::Error::new(typepath.path.span(), "Pack_struct expected at least one path segment!")
            })?.ident;

            Ok(type_ident.to_string())
        }
        _ => Err(syn::Error::new(ty.span(), "Pack_struct does not support fields where the types have lifetimes, types with typeparameters, or other complex types.")),
    }
}

struct SimplifiedField {
    ident: Ident,
    ty: Type,
    size: usize,
}

#[proc_macro_attribute]
pub fn pack_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let original_struct: ItemStruct = parse_macro_input!(item);

    // Grab only the information we want from the syntax tree
    let fields: Result<Vec<SimplifiedField>, syn::Error> = original_struct
        .fields
        .iter()
        .map(|field| {
            if let Some(ident) = field.ident.clone() {
                Ok(SimplifiedField {
                    ident,
                    ty: field.ty.clone(),
                    size: type_to_size(&field.ty)?,
                })
            } else {
                Err(syn::Error::new(
                    field.span(),
                    "Pack_struct does not support unnamed fields (tuples).",
                ))
            }
        })
        .collect();

    let fields = match fields {
        Ok(fields) => fields,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    // Calculate total size of struct, ceil to next power of 2.
    let total_size: usize = fields.iter().map(|field| field.size).sum();
    let mut total_size: usize = usize::pow(2, ((total_size as f64).ln() / 2f64.ln()).ceil() as u32);

    if total_size < 8 {
        total_size = 8;
    } else if total_size > 128 {
        return syn::Error::new(Span::call_site(), format!("Fields in struct exceed 128 bit and cannot be packed into a single unsigned integer, current size: {}", total_size)).to_compile_error().into();
    }

    // Implement needed types and methods
    let data_field_type = syn::Ident::new(&format!("u{}", total_size), Span::call_site());

    let pack_impl = fields.iter().enumerate().map(|(i, SimplifiedField{ident, ty, size})|
        {
            let ty_as_unsigned_integer = syn::Ident::new(&format!("u{}", std::cmp::max(8, *size)), Span::call_site());

            if i == 0{
                quote! {
                    packed.data |= std::mem::transmute_copy::<#ty, #ty_as_unsigned_integer>(&self.#ident) as #data_field_type;
                }
            } else {
                quote! {
                    packed.data <<= #size;
                    packed.data |= std::mem::transmute_copy::<#ty, #ty_as_unsigned_integer>(&self.#ident) as #data_field_type;
                }
            }
        }
    );

    let unpack_impl = fields.iter().rev().enumerate().map(|(i, SimplifiedField{ident, ty, size})|
        {
            let ty_as_unsigned_integer = syn::Ident::new(&format!("u{}", std::cmp::max(8, *size)), Span::call_site());

            if i == fields.len() - 1 {
                quote! {
                    unpacked.#ident = std::mem::transmute_copy::<#ty_as_unsigned_integer, #ty>(&(data_copy as #ty_as_unsigned_integer));
                }
            } else {
                quote! {
                    unpacked.#ident = std::mem::transmute_copy::<#ty_as_unsigned_integer, #ty>(&(data_copy as #ty_as_unsigned_integer));
                    data_copy >>= #size;
                }
            }
        }
    );

    let original_struct_ident = &original_struct.ident;
    let packed_struct_ident = syn::Ident::new(
        &format!("{}Packed", original_struct.ident.to_string()),
        Span::call_site(),
    );

    TokenStream::from(quote! {
        #original_struct

        #[derive(Copy, Clone)]
        struct #packed_struct_ident {
            data: #data_field_type
        }

        impl #original_struct_ident {
            pub fn pack(&self) -> #packed_struct_ident {
                let mut packed = #packed_struct_ident{data: 0};

                unsafe {
                    #(#pack_impl)*
                }

                return packed;
            }
        }

        impl #packed_struct_ident {
            pub fn unpack(&self) -> #original_struct_ident {
                let mut data_copy = self.data;
                let mut unpacked = #original_struct_ident::default();

                unsafe {
                    #(#unpack_impl)*
                }
                return unpacked;
            }
        }
    })
    .into()
}

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
use proc_macro2::{Span};

fn ty_to_ident(ty: &syn::Type) -> Option<&syn::Ident>{
    if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = ty {
        if let Some(syn::PathSegment{ref ident, ..}) = segments.last() {
            Some(ident)
        } else {
            None
        }
    } else {
        None
    }
}

fn primitive_to_size(ty: &syn::Type) -> Option<usize> {
    let ty_ident = if let Some(ident) = ty_to_ident(&ty) {
        ident
    } else {
        return None;
    };

    match ty_ident.to_string().as_str() {
        "bool" => Some(1),
        "u8" | "i8" => Some(8),
        "u16" | "i16" => Some(16),
        "char" | "u32" | "i32" | "f32" => Some(32),
        "u64" | "i64" | "f64" => Some(64),
        "u128" | "i128" => Some(128),
        "usize" => Some(std::mem::size_of::<usize>()),
        "isize" => Some(std::mem::size_of::<isize>()),
        _ => None
    }
}

#[proc_macro_attribute]
pub fn pack_struct(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse input
    let original_struct = parse_macro_input!(input as DeriveInput);

    // Create list of syn::Fields's
    let fields = if let syn::Data::Struct(syn::DataStruct {
                        fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}), ..})
                 = original_struct.data
                {
                    Option::Some(named.clone())
                } else {
                    Option::None
                };

    let fields = match &fields {
        Some(fields) => fields,
        None => {panic!("Couldent find any fields in struct!");}
    };

    // Calculate size of packed data
    let struct_size: usize = fields.iter().map(
        |field| {
            let opt = primitive_to_size(&field.ty);

            if let Some(size) = opt {
                size
            } else {
                panic!("Couldent find size of primitive type!");
            }
        }
    ).sum();
        // Find next higher(or equal) power of 2
    let mut data_size: usize = usize::pow(2, ((struct_size as f64).ln()/2f64.ln()).ceil() as u32);


    if data_size < 8 {
        data_size = 8;
    } else if data_size > 128 {
        panic!("Fields in struct cannot be packed in a unsigned integer! Max bit size of fields is 128, actual: {}", struct_size);
    }

    let data_field_type = syn::Ident::new(&format!("u{}", data_size), Span::call_site());

    let pack_impl = fields.iter().enumerate().map(|(i, field)|
        {
            let field_ident = field.ident.as_ref();
            let field_ident = if let Some(ident) = field_ident {
                ident
            } else {
                panic!("Macro does not support unnamed types!");
            };

            let ty = &field.ty;

            let ty_size = primitive_to_size(ty);
            let ty_size = if let Some(integer) = ty_size {
                integer
            } else {
                panic!("Couldent find type size!");
            };

            let ty_as_unsigned_integer = if let Some(ident) = ty_to_ident(&ty) {
                if ident.to_string() == "bool" {
                    syn::Ident::new(&format!("u{}", 8), Span::call_site())
                } else {
                    syn::Ident::new(&format!("u{}", ty_size), Span::call_site())
                }
            } else {
                panic!("Couldent find type ident!");
            };

            if i == 0{
                quote! {
                    packed.data |= std::mem::transmute_copy::<#ty, #ty_as_unsigned_integer>(&self.#field_ident) as #data_field_type;
                }
            } else {
                quote! {
                    packed.data <<= #ty_size;
                    packed.data |= std::mem::transmute_copy::<#ty, #ty_as_unsigned_integer>(&self.#field_ident) as #data_field_type;
                }
            }
        }
    );

    let unpack_impl = fields.iter().rev().enumerate().map(|(i, field)|
        {
            let field_ident = field.ident.as_ref();
            let field_ident = if let Some(ident) = field_ident {
                ident
            } else {
                panic!("Macro does not support unnamed types!");
            };

            let ty = &field.ty;

            let ty_size = primitive_to_size(&field.ty);
            let ty_size = if let Some(integer) = ty_size {
                integer
            } else {
                panic!("Couldent find type size!");
            };

            let ty_as_unsigned_integer = if let Some(ident) = ty_to_ident(&ty) {
                if ident.to_string() == "bool" {
                    syn::Ident::new(&format!("u{}", 8), Span::call_site())
                } else {
                    syn::Ident::new(&format!("u{}", ty_size), Span::call_site())
                }
            } else {
                panic!("Couldent find type ident!");
            };

            if i == fields.len() - 1 {
                quote! {
                    unpacked.#field_ident = std::mem::transmute_copy::<#ty_as_unsigned_integer, #ty>(&(data_copy as #ty_as_unsigned_integer));
                }
            } else {
                quote! {
                    unpacked.#field_ident = std::mem::transmute_copy::<#ty_as_unsigned_integer, #ty>(&(data_copy as #ty_as_unsigned_integer));
                    data_copy >>= #ty_size;
                }
            }
        }
    );

    let original_struct_ident = &original_struct.ident;
    let packed_struct_ident = syn::Ident::new(&format!("{}Packed", original_struct.ident.to_string()), Span::call_site());

    TokenStream::from(
        quote!{
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
        }
    ).into()
}
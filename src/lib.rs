extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
use proc_macro2::{Span};

fn ty_is_float(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = ty {
        if let Some(syn::PathSegment{ref ident, ..}) = segments.last() {
            ident.to_string().contains('f')
        } else {
            false
        }
    } else {
        false
    }
}

fn ty_is_integer(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = ty {
        if let Some(syn::PathSegment{ref ident, ..}) = segments.last() {
            ident.to_string().contains('i')
        } else {
            false
        }
    } else {
        false
    }
}

fn ty_is_char(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = ty {
        if let Some(syn::PathSegment{ref ident, ..}) = segments.last() {
            ident.to_string().contains("char")
        } else {
            false
        }
    } else {
        false
    }
}

fn field_to_ty_ident(ty: &syn::Type) -> Option<&syn::Ident>{
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
    let ty_ident = if let Some(ident) = field_to_ty_ident(&ty) {
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
    // Parse args (TODO)

    // Parse input
        // Create list of syn::Field's
            // TODO: support unnamed fields (tuple structs)

    // Write the type/name of data field which holds all the fields in the original struct.
        // Type should be unsigned integer, with bits being size_sum rounded up to 2^x, maximum u128.
        // Return compile error if size exceeds 128 bits

    // Write the packing logic

    // Write the unpacking logic

    // Derive from Copy, Clone, Debug

    // Implement PartialEq and PartialOrd

    // Write the final result to token stream
        // Write the original struct, OriginalStruct
        // Write the new struct, OriginalStructPacked
        // Write OriginalStruct implementation for packing
        // Write OriginalStructPacked implementation for unpacking
        // Write OriginalStructPacked implementation of PartialEq and PartialOrd


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
    let mut data_size: usize = usize::pow(2, ((struct_size as f64).ln()/2f64.ln()).ceil() as u32);


    if data_size < 8 {
        data_size = 8;
    } else if data_size > 128 {
        panic!("Fields in struct cant be packed in a unsigned integer! Max bit size of fields is 128, actual: {}", struct_size);
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

            if ty_is_float(&ty) {
                let float_as_unsigned_ident = syn::Ident::new(&format!("u{}", ty_size), Span::call_site());
                if i == 0{
                    quote! {
                        packed.data |= #float_as_unsigned_ident::from_ne_bytes(self.#field_ident.to_ne_bytes()) as #data_field_type;
                    }
                } else {
                    quote! {
                        packed.data <<= #ty_size;
                        packed.data |= #float_as_unsigned_ident::from_ne_bytes(self.#field_ident.to_ne_bytes()) as #data_field_type;
                    }
                }
            } else if ty_is_char(&ty) {
                if i == 0{
                    quote! {
                        packed.data |= u32::from(self.#field_ident) as #data_field_type;
                    }
                } else {
                    quote! {
                        packed.data <<= #ty_size;
                        packed.data |= u32::from(self.#field_ident) as #data_field_type;
                    }
                }
            } else if ty_is_integer(&ty) {
                let integer_as_unsigned_ident = syn::Ident::new(&format!("u{}", ty_size), Span::call_site());
                if i == 0{
                    quote! {
                        packed.data |= #integer_as_unsigned_ident::from_ne_bytes(self.#field_ident.to_ne_bytes()) as #data_field_type;
                    }
                } else {
                    quote! {
                        packed.data <<= #ty_size;
                        packed.data |= #integer_as_unsigned_ident::from_ne_bytes(self.#field_ident.to_ne_bytes()) as #data_field_type;
                    }
                }
            } else {
                if i == 0{
                    quote! {
                        packed.data |= self.#field_ident as #data_field_type;
                    }
                } else {
                    quote! {
                        packed.data <<= #ty_size;
                        packed.data |= self.#field_ident as #data_field_type;
                    }
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

            if ty_is_float(&ty) {
                let float_as_unsigned_ident = syn::Ident::new(&format!("u{}", ty_size), Span::call_site());

                if i == fields.len() - 1 {
                    quote! {
                        unpacked.#field_ident = #ty::from_ne_bytes((data_copy as #float_as_unsigned_ident).to_ne_bytes());
                    }
                } else {
                    quote! {
                        unpacked.#field_ident = #ty::from_ne_bytes((data_copy as #float_as_unsigned_ident).to_ne_bytes());
                        data_copy >>= #ty_size;
                    }
                }
            } else if ty_is_char(&ty) {
                if i == fields.len() - 1 {
                    quote! {
                        unpacked.#field_ident = char::from_u32_unchecked(data_copy as u32);
                    }
                } else {
                    quote! {
                        unpacked.#field_ident = char::from_u32_unchecked(data_copy as u32);
                        data_copy >>= #ty_size;
                    }
                }
            } else if ty_is_integer(&ty) {
                let integer_as_unsigned_ident = syn::Ident::new(&format!("u{}", ty_size), Span::call_site());
                if i == fields.len() - 1 {
                    quote! {
                        unpacked.#field_ident = #ty::from_ne_bytes((data_copy as #integer_as_unsigned_ident).to_ne_bytes());
                    }
                } else {
                    quote! {
                        unpacked.#field_ident = #ty::from_ne_bytes((data_copy as #integer_as_unsigned_ident).to_ne_bytes());
                        data_copy >>= #ty_size;
                    }
                }
            } else {
                if i == fields.len() - 1 {
                    quote! {
                        unpacked.#field_ident = data_copy as #ty;
                    }
                } else {
                    quote! {
                        unpacked.#field_ident = data_copy as #ty;
                        data_copy >>= #ty_size;
                    }
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
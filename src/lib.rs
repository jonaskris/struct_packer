extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
use proc_macro2::{Span};

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
        "char" => Some(4),
        "u8" | "i8" => Some(8),
        "u16" | "i16" => Some(16),
        "u32" | "i32" | "f32" => Some(32),
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
    //println!("{:#?}", original_struct);

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
    println!("data_size: {}", data_size);

    let data_field_type = syn::Ident::new(&format!("u{}", data_size), Span::call_site());


    let pack_impl = fields.iter().enumerate().map(|(i, field)|
        {
            let field_ident = field.ident.as_ref();
            let field_ident = if let Some(ident) = field_ident {
                ident
            } else {
                panic!("Macro does not support unnamed types!");
            };

            let ty_size = primitive_to_size(&field.ty);
            let ty_size = if let Some(integer) = ty_size {
                integer
            } else {
                panic!("Couldent find type size!");
            };

            eprintln!("{}", ty_size);

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

            if i == fields.len() - 1 {
                quote! {
                    unpacked.#field_ident |= data_copy as #ty;
                }
            } else {
                quote! {
                    unpacked.#field_ident |= data_copy as #ty;
                    data_copy >>= #ty_size;
                }
            }
        }
    );

    let ret = quote!{
        #original_struct

        #[derive(Copy, Clone)]
        struct SomeStructPacked {
            data: #data_field_type
        }

        impl SomeStruct {
            pub fn pack(&self) -> SomeStructPacked {
                let mut packed = SomeStructPacked{data: 0};

                #(#pack_impl)*

                return packed;
            }
        }

        impl SomeStructPacked {
            pub fn unpack(&self) -> SomeStruct {
                let mut data_copy = self.data;
                let mut unpacked = SomeStruct::default();
                #(#unpack_impl)*
                return unpacked;
            }
        }
    };

    eprintln!("\n\n{}", ret);

    TokenStream::from(
        ret
    ).into()



    /*let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    eprintln!("{:#?}", input);
    let mut fieldBitsSize = 0;

    let fields = if let syn::Data::Struct(syn::DataStruct {
                        fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}), ..})
                 = input.data
    {
        named
    } else {
        unimplemented!();
    };

    let mut fields_types = Vec::new();

    for field in fields {
        let field_ident = if let
            syn::Field{
                // Match field_ident
                    ident: Some(ref field_ident),
                // Match type_ident
                    ty: syn::Type::Path(syn::TypePath{
                        path: syn::Path{segments, ..},
                        ..})
            , ..} = field
            {
                (field_ident, segments)
            } else {
                unimplemented!();
            };
        fields_types.push(field_ident);
    }

    eprintln!("fields_types len: {}", fields_types.len());

    for (field_name, field_type) in fields_types {
        eprintln!("Field name: {:?}, Field type: {:?}", field_name, field_type);
    }

    /*for field in fields {
        fields_types.push(
            if let syn::Field{ty: syn::TypePath{path: syn::Path{segments: syn::PathSegment{ident: syn::Ident{ref ident, ..}, ..}, ..}, ..}, ..} = field
            {
                ident
            } else
            {
                unimplemented!();
            }
        );
    }

    for field_type in fields_types {
        println!("type: {}", field_type);
    }*/

    //println!("size: {}", fields.length());

    /*for field in fields {


        println!("{}", ty_size);
    }*/

    TokenStream::from(quote!{struct SomeStruct {
        #fields
    }}).into()*/

}
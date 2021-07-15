extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, spanned};

#[proc_macro_attribute]
pub fn pack_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse args (TODO)

    // Parse input
        // Create list of syn::Field's
            // TODO: support unnamed fields (tuple structs)

    // Write the type/name of data field which holds all the fields in the original struct.
        // Implement const DATA_BITS which is the number of bits the data takes up.
        // Implement const DATA_BITS_ROUNDED which si rounded to the nearest 2^x, maximum 128
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

    println!("{:#?}", original_struct);

        // Create list of syn::Fields's
    let fields = if let syn::Data::Struct(syn::DataStruct {
                        fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}), ..})
                 = original_struct.data
                {
                    Option::Some(named.clone())
                } else {
                    Option::None
                };

    let fields = match fields {
        Some(fields) => fields,
        None => unimplemented!()
    };

    let fields_types: Vec<&syn::Type> = fields.iter().map(|field| &field.ty).collect();

    let test = quote!{
        const DATA_SIZE: u8 = #(std::mem::size_of::<#fields_types>())+*;
        const DATA_SIZE_ROUNDED: u8 = u8::pow(2, ceil(log(DATA_SIZE)/log(2)));
    };


    println!("quote: {}", test);



        // Implement const DATA_BITS which is the number of bits the data takes up.
    /*let c_data_bits = quote!{
        const DATA_BITS = #( std::mem:size_of<#fields> + )*;
    };*/

    //println!("c_data_bits: {:#?}", c_data_bits);
        // Implement const DATA_BITS_ROUNDED which si rounded to the nearest 2^x, maximum 128


    //println!("{:#?}", original_struct);




    let ret = quote!{
        #original_struct

        struct SomeStructPacked {
            #test
        }
    };

    println!("\n\n{}", ret);

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
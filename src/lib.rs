extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
use proc_macro2::{Span};

fn primitive_to_size(ty: &syn::Type) -> usize {
    let last_segment: Option<&syn::PathSegment> = if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = ty {
        segments.last()
    } else {
        panic!("Struct_packer does not support non-primitive or compound types!");
    };

    let last_segment = if let Some(segment) = last_segment {
        segment
    } else {
        panic!("Struct_packer does not support non-primitive or compound types!");
    };

    match last_segment.ident.to_string().as_str() {
        "bool" => 1,
        "char" => 4,
        "u8" | "i8" => 8,
        "u16" | "i16" => 16,
        "u32" | "i32" | "f32" => 32,
        "u64" | "i64" | "f64" => 64,
        "u128" | "i128" => 128,
        "usize" => std::mem::size_of::<usize>(),
        "isize" => std::mem::size_of::<isize>(),
        _ => {panic!("Struct_packer does not support non-primitive or compound types!")}
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

    let fields = match &fields {
        Some(fields) => fields,
        None => {panic!("Couldent find any fields in struct!");}
    };

    // Calculate size of packed data
    let struct_size: usize = fields.iter().map(|field| primitive_to_size(&field.ty)).sum();
    let mut data_size: usize = usize::pow(2, ((struct_size as f64).ln()/2f64.ln()).ceil() as u32);


    if data_size < 8 {
        data_size = 8;
    } else if data_size > 128 {
        panic!("Fields in struct cant be packed in a unsigned integer! Max bit size of fields is 128, actual: {}", struct_size);
    }
    println!("data_size: {}", data_size);

    //let data_field = syn::Ident::new("data: {}", Span::call_site());
    let data_field_type = syn::Ident::new(&format!("u{}", data_size), Span::call_site());

    //let fields_types: Vec<&syn::Type> = fields.iter().map(|field| &field.ty).collect();

    /*let test = quote!{
        const DATA_SIZE: u8 = #(std::mem::size_of::<#fields_types>())+*;
        const DATA_SIZE_ROUNDED: u8 = u8::pow(2, ceil(log(DATA_SIZE)/log(2)));
    };*/

    /*for field_type in fields_types {
        println!("ty: {:?}", field_type);
    }*/

    //println!("{:#?}", original_struct);

    //println!("quote: {}", test);



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
            data: #data_field_type
        }

        impl SomeStruct {
            pub fn pack(&self) -> SomeStructPacked {
                
            }
        }
    };

    //println!("\n\n{}", ret);

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
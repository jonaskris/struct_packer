extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};

fn print_size_of_type<T>(_: &T)
{
    println!("{} size: {}", std::any::type_name::<T>(), std::mem::size_of::<T>())
}

#[proc_macro_attribute]
pub fn bit_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
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
    }}).into()

}
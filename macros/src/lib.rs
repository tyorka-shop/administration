extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, Lit, Meta, MetaNameValue,
};

mod consts;
mod get_all;
mod get_by_id;
mod insert;
mod insert_or_update;
mod insert_all;
mod clear;

fn get_table_name(attrs: Vec<Attribute>) -> String {
    let meta = attrs.get(0).unwrap().parse_meta().unwrap();
    if let Meta::NameValue(MetaNameValue { path, lit, .. }) = meta {
        if path.is_ident("table_name") {
            if let Lit::Str(lit_str) = lit {
                return lit_str.value();
            }
        }
    }
    panic!("expected table_name attribute")
}

#[proc_macro_derive(Entity, attributes(table_name))]
pub fn derive_from_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    // Attributes -> field names
    let fields = fields.iter().filter_map(|field| field.ident.clone()).collect::<Vec<Ident>>();

    let struct_name = &input.ident;

    let table_name = get_table_name(input.attrs);

    let get_by_id = get_by_id::make(struct_name, &table_name);

    let get_all = get_all::make(struct_name, &table_name);

    let insert = insert::make(&table_name, &fields);

    let insert_all = insert_all::make(&table_name, &fields);

    let insert_or_update = insert_or_update::make(&table_name, &fields);

    let clear = clear::make(&table_name);

    TokenStream::from(quote! {
        impl #struct_name {

            #get_by_id

            #get_all

            #insert

            #insert_all

            #insert_or_update

            #clear
        }
    })
}

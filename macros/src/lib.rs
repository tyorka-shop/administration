extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, Lit, Meta, MetaNameValue,
};

fn placeholders(max: usize) -> String {
    (1..max + 1)
        .into_iter()
        .map(|s| format!("${}", s))
        .collect::<Vec<String>>()
        .join(", ")
}

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
    let fields_for_query = fields.iter().map(|field| &field.ident);
    let fields_for_insert = fields.iter().map(|field| &field.ident);
    let fields_for_replace = fields.iter().map(|field| &field.ident);
    let fields_for_ignore = fields.iter().map(|field| &field.ident);

    let struct_name = &input.ident;

    let field_length = fields_for_query.len();
    // ( $1, $2)
    let values = placeholders(field_length);

    let fields_list = quote! {
        #( #fields_for_query ),*
    };
    let table_name = get_table_name(input.attrs);

    let select_all_sql = format!("SELECT * FROM `{}`", table_name);
    let select_by_id_sql = format!("SELECT * FROM `{}` WHERE id = $1", table_name);

    let insert_sql = format!(
        "insert into `{}` ( {} ) values ( {} )",
        table_name, fields_list, values
    );

    let insert_or_ignore_sql = format!(
        "insert or ignore into `{}` ( {} ) values ( {} )",
        table_name, fields_list, values
    );

    let replace_sql = format!(
        "replace into `{}` ( {} ) values ( {} )",
        table_name, fields_list, values
    );

    TokenStream::from(quote! {
        impl #struct_name {

            pub async fn get_by_id(pool: &sqlx::SqlitePool, id: &str) -> Result<#struct_name, sqlx::Error> {
                sqlx::query_as!(#struct_name, #select_by_id_sql, id)
                    .fetch_one(pool)
                    .await
                    .map(|row| row.into())
            }

            pub async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<#struct_name>, sqlx::Error> {
                sqlx::query_as!(#struct_name, #select_all_sql)
                    .fetch_all(pool)
                    .await
                    .map(|rows| rows.into_iter().map(|row| row.into()).collect())
            }

            pub async fn insert(&self, pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
                Ok(sqlx::query!(#insert_sql,
                #(
                    self.#fields_for_insert,
                )*
                    ).execute(pool)
                    .await?
                )
            }

            pub async fn insert_or_ignore(&self, pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
                Ok(sqlx::query!(#insert_or_ignore_sql,
                #(
                    self.#fields_for_ignore,
                )*
                    ).execute(pool)
                    .await?
                )
            }
            pub async fn insert_or_replace(&self, pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
                Ok(sqlx::query!(#replace_sql,
                #(
                    self.#fields_for_replace,
                )*
                    ).execute(pool)
                    .await?
                )
            }
        }
    })
}

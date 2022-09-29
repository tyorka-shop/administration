use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::consts::{CREATED_AT, UPDATED_AT};

pub fn make(table_name: &str, fields: &Vec<Ident>) -> TokenStream {
    let mut idx = 0;
    let (names, placeholders): (Vec<String>, Vec<String>) = fields
        .iter()
        .map(|field| match field.to_string().as_str() {
            CREATED_AT | UPDATED_AT => (format!("`{field}`"), "datetime('now')".to_string()),
            _ => {
                idx = idx + 1;
                return (format!("`{field}`"), format!("${}", idx));
            }
        })
        .unzip();

    let filtered_fileds = fields
        .iter()
        .filter(|field| match field.to_string().as_str() {
            CREATED_AT | UPDATED_AT => false,
            _ => true,
        })
        .collect::<Vec<&Ident>>();

    let insert_statement = format!(
        "insert into `{}` ( {} ) values ( {} )",
        table_name,
        names.join(", "),
        placeholders.join(", ")
    );

    let insert_or_ignore_statement = format!(
        "insert or ignore into `{}` ( {} ) values ( {} )",
        table_name,
        names.join(", "),
        placeholders.join(", ")
    );


    quote! {
        pub async fn insert(&self, pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
            Ok(sqlx::query!(#insert_statement,
            #(
                self.#filtered_fileds,
            )*
                ).execute(pool)
                .await?
            )
        }

        pub async fn insert_or_ignore(&self, pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
            Ok(sqlx::query!(#insert_or_ignore_statement,
            #(
                self.#filtered_fileds,
            )*
                ).execute(pool)
                .await?
            )
        }
    }
}

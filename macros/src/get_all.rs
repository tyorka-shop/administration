use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn make(struct_name: impl ToTokens, table_name: &str) -> TokenStream {
    let statement = format!("SELECT * FROM `{}`", table_name);

    quote! {
        pub async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<#struct_name>, sqlx::Error> {
            sqlx::query_as!(#struct_name, #statement)
                .fetch_all(pool)
                .await
                .map(|rows| rows.into_iter().map(|row| row.into()).collect())
        }
    }
}

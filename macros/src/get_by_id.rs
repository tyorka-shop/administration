use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn make(struct_name: impl ToTokens, table_name: &str) -> TokenStream {
    let statement = format!("SELECT * FROM `{}` WHERE id = $1", table_name);

    quote! {
        pub async fn get_by_id(pool: &sqlx::SqlitePool, id: &str) -> Result<#struct_name, sqlx::Error> {
            sqlx::query_as!(#struct_name, #statement, id)
                .fetch_one(pool)
                .await
                .map(|row| row.into())
        }
    }
}

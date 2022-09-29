use proc_macro2::TokenStream;
use quote::quote;

pub fn make(table_name: &str) -> TokenStream {
    let statement = format!("delete from `{}`", table_name);
    quote! {
        pub async fn clear(pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
            Ok(sqlx::query!(#statement).execute(pool).await?)
        }
    }
}

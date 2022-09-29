use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn make(table_name: &str, fields: &Vec<Ident>) -> TokenStream {
    let (names, placeholders): (Vec<String>, Vec<String>) = fields
        .iter()
        .enumerate()
        .map(|(idx, field)| (format!("`{field}`"), format!("${}", idx + 1)))
        .unzip();

    let insert_statement = format!(
        "insert into `{}` ( {} ) values ( {} )",
        table_name,
        names.join(", "),
        placeholders.join(", ")
    );

    quote! {
        pub async fn insert_all(&self, pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
            Ok(sqlx::query!(#insert_statement,
            #(
                self.#fields,
            )*
                ).execute(pool)
                .await?
            )
        }
    }
}

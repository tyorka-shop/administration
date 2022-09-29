use proc_macro2::{TokenStream, Ident};
use quote::quote;

use crate::consts::{CREATED_AT, UPDATED_AT, ID};


pub fn make(table_name: &str, fields: &Vec<Ident>) -> TokenStream {
    let mut idx = 1; //skip for id
    let (assignments, fields): (Vec<String>, Vec<Option<Ident>>) = fields
        .into_iter()
        .filter(|field| *field != ID && *field != CREATED_AT)
        .map(|field| match field.to_string().as_str() {
            UPDATED_AT => (format!("`{field}` = datetime('now')"), None),
            _ => {
                idx = idx + 1;
                return (format!("`{field}` = ${idx}"), Some(field.clone()));
            }
        })
        .unzip();

    let statement = format!(
        "update `{}` set {} where id = $1",
        table_name, assignments.join(", ")
    );

    let fields = fields.iter().filter_map(|e| e.as_ref()).collect::<Vec<&Ident>>();

    quote! {
        pub async fn insert_or_update(&self, pool: &sqlx::SqlitePool) -> eyre::Result<sqlx::sqlite::SqliteQueryResult> {
            let result = sqlx::query!(#statement,
                self.id,
            #(
                self.#fields,
            )*
                ).execute(pool)
                .await?;

            if result.rows_affected() > 0 {
                return Ok(result);
            }

            self.insert(pool).await
        }
    }
}

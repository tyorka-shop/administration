use serde::{ Serialize};
use tokio::task::JoinHandle;
use entity::BlogPost;
use crate::types::{Response};
use crate::download::download_media;


const HOST: &str = "https://graph.facebook.com";

const FIELDS: &[&str] = &[
    "media_url",
    "thumbnail_url",
    "caption",
    "media_type",
    "timestamp",
    "children{media_url}",
    "permalink",
];

#[derive(Serialize)]
struct Query {
    fields: String,
    limit: u32,
    access_token: String,
}

pub async fn get_posts(
    access_token: &str,
    instagram_id: &str,
    limit: u32,
    images_folder: &str,
) -> std::io::Result<Vec<BlogPost>> {
    let query = Query {
        fields: FIELDS.join(","),
        limit,
        access_token: access_token.to_string(),
    };

    let query = serde_qs::to_string(&query).unwrap();

    let response = reqwest::get(format!("{}/v12.0/{}/media?{}", HOST, instagram_id, query))
        .await
        .unwrap();

    let response = response.json::<Response>().await.unwrap().data;

    log::debug!("Response: {:?}", &response);

    let posts = response
        .into_iter()
        .map(|post| {
            let to_folder = images_folder.to_string();
            tokio::spawn(async move {
                let img = download_media(&post, &to_folder).await;

                BlogPost {
                    id: post.id.clone().into(),
                    src: img.id().to_string(),
                    url: post.permalink.clone(),
                    color: img.dominant_color(),
                }
            })
        })
        .collect::<Vec<JoinHandle<BlogPost>>>();

    let post = futures::future::join_all(posts)
        .await
        .into_iter()
        .map(|p| p.unwrap())
        .collect();
    Ok(post)
}

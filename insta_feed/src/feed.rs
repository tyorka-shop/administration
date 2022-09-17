use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use tyorka_admin_lib::entity::{blog_post::BlogPost, picture::Picture};

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

#[derive(Deserialize, Serialize, Debug)]
struct Post {
    id: String,
    caption: Option<String>,
    media_type: String,
    media_url: Option<String>,
    permalink: String,
    thumbnail_url: Option<String>,
    timestamp: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    pub data: Vec<Post>,
}

async fn download(url: &str, to_folder: &str) -> Picture {
    let bytes = reqwest::get(url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec();

    Picture::create(&bytes, to_folder).unwrap()
}

async fn download_media(post: &Post, to_folder: &str) -> Picture {
    log::debug!("Download {:?}", post.caption);
    match post.media_type.as_ref() {
        "IMAGE" | "CAROUSEL_ALBUM" => download(&post.media_url.clone().unwrap(), to_folder).await,
        "VIDEO" => download(&post.thumbnail_url.clone().unwrap(), to_folder).await,
        _ => panic!("Unknown media type {}", post.media_type),
    }
}

pub async fn get_posts(
    access_token: &str,
    instagram_id: &str,
    limit: u32,
    images_folder: &str,
) -> () {
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
                let pic = download_media(&post, &to_folder).await;

                BlogPost {
                    id: post.id.clone().into(),
                    src: pic.id.to_string(),
                    url: post.permalink.clone(),
                    color: pic.color,
                }
            })
        })
        .collect::<Vec<JoinHandle<BlogPost>>>();

    futures::future::join_all(posts).await;
}

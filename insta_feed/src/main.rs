use tyorka_admin_lib::init_store;

mod feed;

#[tokio::main]
async fn main() {
    env_logger::init();
    let cfg = config::load("tyorka-admin");

    init_store(&cfg.images_folder).unwrap();

    if let Some(insta) = &cfg.insta {
        feed::get_posts(
            &insta.access_token,
            &insta.instagram_id,
            12,
            &cfg.images_folder,
        )
        .await;
    }
}

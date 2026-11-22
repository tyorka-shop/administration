use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use reqwest::{Client, Method, Response, StatusCode};
use serde::Deserialize;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Builder {
    client: Client,
    start_lock: Arc<Mutex<()>>,
    base_url: String,
    token: Option<String>,
    images_folder: String,
    insta_cfg: Option<config::InstaConfig>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildState {
    Running,
    Stopping,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub id: u64,
    pub state: BuildState,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StatusResponse {
    Build(Build),
    Idle {
        #[serde(rename = "state")]
        _state: IdleState,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum IdleState {
    Idle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryPage {
    builds: Vec<Build>,
    next_before: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LogsPage {
    logs: String,
    last_cursor: u64,
    has_more: bool,
}

impl Builder {
    pub fn new(config: &config::Config) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("Failed to initialize Astro builder HTTP client"),
            start_lock: Arc::new(Mutex::new(())),
            base_url: config.builder_url.trim_end_matches('/').to_owned(),
            token: config.builder_token.clone(),
            images_folder: config.images_folder.clone(),
            insta_cfg: config.insta.clone(),
        }
    }

    async fn request(&self, method: Method, path: &str) -> eyre::Result<Response> {
        let token = self
            .token
            .as_deref()
            .filter(|token| !token.trim().is_empty())
            .ok_or_else(|| eyre::eyre!("Set builder_token or BUILDER_TOKEN"))?;

        Ok(self
            .client
            .request(method, format!("{}{path}", self.base_url))
            .bearer_auth(token)
            .send()
            .await?)
    }

    pub async fn build(&self, db: &SqlitePool) -> eyre::Result<Build> {
        let _guard = self.start_lock.lock().await;
        if let Some(cfg) = &self.insta_cfg {
            eyre::ensure!(
                self.current_build().await?.is_none(),
                "A build is already running"
            );
            if let Err(error) = insta_sync::sync(cfg, &self.images_folder, db).await {
                log::error!("Error while syncing Instagram: {error}");
            }
        }
        Ok(self
            .request(Method::POST, "/build")
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn current_build(&self) -> eyre::Result<Option<Build>> {
        let status: StatusResponse = self
            .request(Method::GET, "/status")
            .await?
            .error_for_status()?
            .json()
            .await?;
        match status {
            StatusResponse::Build(build)
                if matches!(build.state, BuildState::Running | BuildState::Stopping) =>
            {
                Ok(Some(build))
            }
            _ => Ok(None),
        }
    }

    pub async fn publication(&self, id: u64) -> eyre::Result<Option<Build>> {
        let response = self.request(Method::GET, &format!("/builds/{id}")).await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        Ok(Some(response.error_for_status()?.json().await?))
    }

    async fn history_page(&self, before: Option<u64>) -> eyre::Result<HistoryPage> {
        let path = match before {
            Some(cursor) => format!("/builds?before={cursor}"),
            None => "/builds".to_owned(),
        };
        Ok(self
            .request(Method::GET, &path)
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn publications(&self) -> eyre::Result<Vec<Build>> {
        let mut builds = Vec::new();
        let mut before = None;
        loop {
            let page = self.history_page(before).await?;
            builds.extend(page.builds);
            before = page.next_before;
            if before.is_none() {
                return Ok(builds);
            }
        }
    }

    pub async fn latest_successful_build(&self) -> eyre::Result<Option<Build>> {
        let mut before = None;
        loop {
            let page = self.history_page(before).await?;
            if let Some(build) = page
                .builds
                .into_iter()
                .find(|build| matches!(build.state, BuildState::Succeeded))
            {
                return Ok(Some(build));
            }
            before = page.next_before;
            if before.is_none() {
                return Ok(None);
            }
        }
    }

    pub async fn logs(&self, id: u64) -> eyre::Result<String> {
        let mut logs = String::new();
        let mut cursor = 0;
        loop {
            let path = format!("/builds/{id}/logs?after={cursor}");
            let page: LogsPage = self
                .request(Method::GET, &path)
                .await?
                .error_for_status()?
                .json()
                .await?;
            logs.push_str(&page.logs);
            if !page.has_more {
                return Ok(logs);
            }
            cursor = page.last_cursor;
        }
    }
}

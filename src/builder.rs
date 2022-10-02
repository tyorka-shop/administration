use std::{
    ffi::OsString,
    io::{BufRead, BufReader, Error},
    sync::Mutex,
    thread,
};

use sqlx::SqlitePool;

use lazy_static::lazy_static;
use subprocess::{Popen, PopenConfig, Redirection};

#[derive(Debug)]
enum CurrentBuild {
    Running(String),
    Idle,
}

lazy_static! {
    static ref BUILD: Mutex<CurrentBuild> = Mutex::new(CurrentBuild::Idle);
}

#[derive(Clone)]
pub struct Builder {
    pub public_site_folder: String,
    pub images_folder: String,
    pub insta_cfg: Option<config::InstaConfig>,
}

impl Builder {
    pub fn current_build(&self) -> Option<String> {
        let build = BUILD.lock().unwrap();
        match *build {
            CurrentBuild::Running(ref id) => Some(id.to_string()),
            CurrentBuild::Idle => None,
        }
    }

    pub fn build(&self, db: &SqlitePool) -> Result<entity::Build, Error> {
        log::debug!("Starting build");
        let build = lock().unwrap();

        let result = build.clone();
        let db = db.clone();
        let builder = self.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                build.insert(&db).await.unwrap();
            });

            builder.insta_sync(&build.id, &db);

            let mut environment = std::env::vars()
                .map(|(k, v)| (OsString::from(k), OsString::from(v)))
                .collect::<Vec<_>>();
            environment.push((
                OsString::from("XDG_CONFIG_HOME"),
                OsString::from(&builder.public_site_folder),
            ));

            let mut cmd = Popen::create(
                &["make"],
                PopenConfig {
                    stdout: Redirection::Pipe,
                    stderr: Redirection::Merge,
                    cwd: Some(OsString::from(&builder.public_site_folder)),
                    env: Some(environment),
                    ..Default::default()
                },
            )
            .unwrap();
            let reader = BufReader::new(cmd.stdout.take().unwrap());
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if line.len() > 0 {
                            rt.block_on(async {
                                store_output(&db, &build.id, &line).await;
                            });
                        }
                    },

                    Err(e) => {
                        log::error!("Error reading line: {}", e);
                    }
                }
            }

            let is_ok = match cmd.wait().unwrap() {
                subprocess::ExitStatus::Exited(0) => true,
                _ => false,
            };
            unlock();
            rt.block_on(async {
                store_status(&db, &build.id, is_ok).await;
            });
            log::debug!("Build finished");
        });

        Ok(result)
    }

    fn insta_sync(&self, build_id: &str, db: &SqlitePool) {
        match self.insta_cfg {
            Some(ref cfg) => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    store_output(&db, &build_id, &format!("Sync insagram")).await;

                    match insta_sync::sync(&cfg, &self.images_folder, &db).await {
                        Err(e) => {
                            log::error!("Error while syncing instagram: {}", e);
                            store_output(
                                &db,
                                &build_id,
                                &format!("Error while syncing instagram: {}", e),
                            )
                            .await;
                        }
                        _ => {
                            store_output(&db, &build_id, &format!("  finished")).await;
                        }
                    };
                });
            }

            None => {
                log::debug!("No instagram config");
            }
        }
    }
}

fn lock() -> Result<entity::Build, Error> {
    let mut current_build = BUILD
        .lock()
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "Can't lock mutex"))?;
    match *current_build {
        CurrentBuild::Running(ref id) => {
            log::error!("Build {} already running", id);
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Build already running",
            ))
        }
        CurrentBuild::Idle => {
            let build = entity::Build::new();
            *current_build = CurrentBuild::Running(build.id.clone());

            Ok(build)
        }
    }
}

fn unlock() {
    let mut current_build = BUILD.lock().expect("Can't lock mutex");
    *current_build = CurrentBuild::Idle;
}

async fn store_output(db: &SqlitePool, id: &str, buf: &str) {
    sqlx::query!(
        r#"update `build` set `log` = `log` || $1 || x'0a', updated_at = datetime('now') where `id` = $2"#,
        buf,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

async fn store_status(db: &SqlitePool, id: &str, is_ok: bool) {
    let status = if is_ok { "DONE" } else { "FAILED" };
    sqlx::query!(
        r#"update `build` set `status` = $2, updated_at = datetime('now') where `id` = $1"#,
        id,
        status
    )
    .execute(db)
    .await
    .unwrap();
}

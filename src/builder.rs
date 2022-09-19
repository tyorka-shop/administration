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
    public_site_folder: String,
}

impl Builder {
    pub fn new(dir: &str) -> Self {
        Self {
            public_site_folder: dir.to_string(),
        }
    }

    pub fn current_build(&self) -> Option<String> {
        let build = BUILD.lock().unwrap();
        match *build {
            CurrentBuild::Running(ref id) => Some(id.to_string()),
            CurrentBuild::Idle => None,
        }
    }

    pub fn build(&self, db: &SqlitePool) -> Result<String, Error> {
        log::debug!("Starting build");
        let build = lock().unwrap();
        
        let mut x = Popen::create(
            &["make"],
            PopenConfig {
                stdout: Redirection::Pipe,
                stderr: Redirection::Merge,
                cwd: Some(OsString::from(self.public_site_folder.clone())),
                env: Some(vec![(OsString::from("XDG_CONFIG_HOME"), OsString::from(self.public_site_folder.clone()))]),
                ..Default::default()
            },
        )
        .unwrap();

        let reader = BufReader::new(x.stdout.take().unwrap());
        let db = db.clone();
        let build_id = build.id.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                build.insert(&db).await.unwrap();
            });
            for line in reader.lines() {
                store_output(&db, &build.id, &line.unwrap());
            }

            let is_ok = match x.wait().unwrap() {
                subprocess::ExitStatus::Exited(0) => true,
                _ => false,
            };
            unlock();
            store_status(&db, &build.id, is_ok);
            log::debug!("Build finished");
        });

        Ok(build_id)
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

fn store_output(db: &SqlitePool, id: &str, buf: &str) {
    let rt = tokio::runtime::Runtime::new().expect("Can't create runtime for store_output");
    rt.block_on(async {
        sqlx::query!(
            r#"update `build` set `log` = `log` || $1 || x'0a' where `id` = $2"#,
            buf,
            id
        )
        .execute(db)
        .await
        .unwrap();
    });
}

fn store_status(db: &SqlitePool, id: &str, is_ok: bool) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let status = if is_ok { "DONE" } else { "FAILED" };
        sqlx::query!(
            r#"update `build` set `status` = $2 where `id` = $1"#,
            id,
            status
        )
        .execute(db)
        .await
        .unwrap();
    });
}

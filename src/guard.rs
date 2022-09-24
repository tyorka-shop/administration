use async_graphql::Guard;
use async_graphql::{Context, Result};
use async_trait::async_trait;
use constant_time_eq::constant_time_eq;
use log::debug;
use poem::http::HeaderMap;

use crate::cached_session_service::CachedSessionService;

#[derive(PartialEq, Clone)]
pub enum Role {
    Client,
    Admin(String),
}

#[derive(Clone)]
pub struct RoleExctractor {
    secret: String,
    session_service_client: CachedSessionService,
}

impl RoleExctractor {
    pub fn new(secret: &str, session_service_url: &str) -> Self {
        Self {
            secret: secret.to_string(),
            session_service_client: CachedSessionService::new(session_service_url),
        }
    }

    pub async fn is_admin(self: &Self, headers: &HeaderMap) -> Option<String> {
        let auth_by_secret = match headers.get("x-auth") {
            Some(token) => match constant_time_eq(&token.as_bytes(), &self.secret.as_bytes()) {
                true => {
                    debug!("Authenticated by secret");
                    Some("admin with secret".to_string())
                }
                false => None,
            },
            None => None,
        };

        if let Some(email) = auth_by_secret {
            return Some(email);
        }

        match headers.get("authorization") {
            Some(token) => match self
                .session_service_client
                .verify(&token.to_str().unwrap().replace("Bearer", "").trim())
                .await
            {
                Ok(email) => {
                    debug!("Authenticated by access_token ({})", email);
                    Some(email)
                }
                Err(e) => {
                    log::error!("Error while verifying access_token: {:?}", e);
                    None
                },
            },

            None => None,
        }
    }
    pub async fn extract(self: &Self, header: &HeaderMap) -> Role {
        match self.is_admin(header).await {
            Some(email) => Role::Admin(email),
            None => Role::Client,
        }
    }
}

pub struct RoleData(pub Role);

impl RoleData {
    pub fn admin() -> Self {
        RoleData(Role::Admin("admin".to_string()))
    }
}

#[async_trait]
impl Guard for RoleData {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let expect: Role = self.into();
        let actual = ctx.data::<Role>().unwrap();

        match expect {
            Role::Admin(_) => match actual {
                Role::Admin(_) => Ok(()),
                _ => Err("Unauthorized".into()),
            },
            _ => Ok(()),
        }
    }
}

impl From<&RoleData> for Role {
    fn from(role: &RoleData) -> Role {
        role.0.clone()
    }
}

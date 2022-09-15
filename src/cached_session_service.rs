use cache::Cache;
use session_service_grpc::{Client, VerifyError};

#[derive(Clone)]
pub struct CachedSessionService(Cache, Client);

impl CachedSessionService {
    pub fn new(url: &str) -> Self {
        CachedSessionService(Cache::new("session_service"), Client::new(url.into()))
    }
    pub async fn verify(&self, token: &str) -> Result<String, VerifyError> {
        match self.0.get(&token) {
            Some(email) => Ok(email),
            None => match self.1.verify(token).await {
                Ok(email) => {
                    self.0.insert(&token, &email, 5 * 60);
                    Ok(email)
                }
                Err(e) => Err(e),
            },
        }
    }
}

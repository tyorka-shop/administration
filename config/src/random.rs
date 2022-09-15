use rand::{distributions::Alphanumeric, Rng};

pub fn make_secret_key() -> String {
  rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(256)
      .map(char::from)
      .collect()
}

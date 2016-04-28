use super::provider::Provider;

pub struct Application {
  provider: Provider
}

impl Application {
  pub fn new(provider: Provider) -> Application {
    Application {
      provider: provider
    }
  }

  pub fn provider(&self) -> &Provider {
      &self.provider
  }
}

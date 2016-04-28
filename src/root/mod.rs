pub mod application;
pub mod database;
pub mod daemon;
pub mod provider;
pub mod settings;

use self::application::Application;
use self::daemon::Daemon;
use self::database::Database;
use self::provider::Provider;
use self::settings::Settings;

pub struct Root {
  application: Application,
  daemon: Daemon,
  settings: Settings
}

impl Root {
  pub fn new() -> Root {
    let settings = Settings::new();
    let database = Database::new(&settings);
    let provider = Provider::new(database);
    let daemon = Daemon::new(&settings, &provider);
    let application = Application::new(provider);

    Root {
      application: application,
      daemon: daemon,
      settings: settings
    }
  }

  pub fn application(&self) -> &Application {
    &self.application
  }

  pub fn database(&self) -> &Database {
    &self.provider().database()
  }

  pub fn daemon(&self) -> &Daemon {
    &self.daemon
  }

  pub fn provider(&self) -> &Provider {
    &self.application.provider()
  }

  pub fn settings(&self) -> &Settings {
    &self.settings
  }
}

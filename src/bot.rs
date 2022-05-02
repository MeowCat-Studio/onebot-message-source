use std::{ops::Deref, sync::Arc};

use mesagisto_client::LateInit;
use walle_core::app::ArcBot;
use walle_v11::{app::OneBot11, Action, Resp};
#[derive(Singleton, Default)]
pub struct OneBot {
  inner: LateInit<Arc<OneBot11>>,
}
impl OneBot {
  pub fn init(&self, bot: Arc<OneBot11>) {
    self.inner.init(bot)
  }
}
impl Deref for OneBot {
  type Target = Arc<OneBot11>;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

#[derive(Singleton, Default)]
pub struct Bot {
  inner: LateInit<ArcBot<Action, Resp>>,
}
impl Bot {
  pub fn init(&self, bot: ArcBot<Action, Resp>) {
    self.inner.init(bot)
  }
}
impl Deref for Bot {
  type Target = ArcBot<Action, Resp>;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}


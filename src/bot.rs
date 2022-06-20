use std::{ops::Deref, sync::Arc};

use mesagisto_client::LateInit;
use walle_core::{BaseEvent, EventContent, Resp, RespContent, StandardAction};

type AliasWalleBot = Arc<
  walle_core::app::OneBot<
    BaseEvent<EventContent>,
    StandardAction,
    Resp<RespContent<BaseEvent<EventContent>>>,
    MyHandler,
    12,
  >,
>;

use crate::handlers::MyHandler;

#[derive(Singleton, Default)]
pub struct OneBot {
  inner: LateInit<AliasWalleBot>,
}
impl OneBot {
  pub fn init(&self, bot: AliasWalleBot) {
    self.inner.init(bot)
  }
}
impl Deref for OneBot {
  type Target = AliasWalleBot;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

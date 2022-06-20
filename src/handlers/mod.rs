pub mod recv;
pub mod send;

use crate::bot::ONE_BOT;
use async_trait::async_trait;
use colored::*;
use tracing::trace;
use tracing::{debug, info};
use walle_core::EventContent;
use walle_core::EventType;
use walle_core::StandardEvent;
use walle_core::{app::ArcBot, EventHandler};

pub struct MyHandler;

#[async_trait]
impl<A, R> EventHandler<StandardEvent, A, R> for MyHandler
where
  A: Send + Sync + 'static,
  R: Send + Sync + 'static,
{
  async fn handle(&self, _: ArcBot<A, R>, event: StandardEvent) {
    match &event.content {
      EventContent::Meta(m) => debug!(
        "[{}] MetaEvent -> Type {}",
        event.self_id.red(),
        m.detail_type().green()
      ),
      EventContent::Message(m) => {
        let alt = if m.alt_message.is_empty() {
          let mut t = format!("{:?}", m.message);
          if t.len() > 15 {
            let _ = t.split_off(15);
          }
          t
        } else {
          m.alt_message.clone()
        };
        info!(
          "[{}] MessageEvent -> from {} alt {}",
          event.self_id.red(),
          m.user_id.blue(),
          alt.green()
        )
      }
      EventContent::Notice(_) => trace!("[{}] NoticeEvent ->", event.self_id.red()),
      EventContent::Request(_) => info!("[{}] RequestEvent ->", event.self_id.red()),
    }
  }
}

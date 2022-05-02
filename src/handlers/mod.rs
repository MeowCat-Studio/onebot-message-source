pub mod recv;
pub mod send;

use crate::bot::BOT;
use crate::bot::ONE_BOT;
use async_trait::async_trait;
use colored::*;
use tracing::{debug, info};
use walle_core::{app::ArcBot, EventHandler};
use walle_v11::{
  event::{EventContent, MetaContent},
  Event,
};

pub struct MyHandler;

#[async_trait]
impl<A, R> EventHandler<Event, A, R> for MyHandler
where
  A: Send + 'static,
  R: Send + 'static,
{
  async fn handle(&self, _: ArcBot<A, R>, event: Event) {
    match event.content {
      EventContent::Message(msg_c) => {
        info!(target: "Walle-core", "[{}] Message -> from {}: {}", event.self_id.to_string().red(), msg_c.user_id.to_string().blue(), msg_c.raw_message.green());
        match msg_c.sub {
          walle_v11::event::MessageSub::Private { sender } => {
            info!(target: "Walle-core", "[{}] Private Message -> from {}: {}", event.self_id.to_string().red(), sender.user_id.to_string().blue(), msg_c.raw_message.green());
          }
          walle_v11::event::MessageSub::Group { group_id, sender } => {
            info!(target: "Walle-core", "[{}] Group Message -> from {} in {}: {}", event.self_id.to_string().red(), sender.user_id.to_string().blue(), group_id.to_string().blue(), msg_c.raw_message.green());
          }
        }
      }
      EventContent::MetaEvent(meta_c) => match meta_c {
        MetaContent::Lifecycle { sub_type: _ } => {
          debug!(target: "oms", "Bot[{}] 已连接", event.self_id.to_string().red());
          let bot = ONE_BOT
            .get_bot(event.self_id.to_string().as_str())
            .await
            .unwrap();
          BOT.init(bot);
        }
        MetaContent::Heartbeat {
          status: _,
          interval: _,
        } => {}
      },
      _ => {}
    }
  }
}

#![allow(incomplete_features)]
#![feature(backtrace, capture_disjoint_fields)]

mod bot;
mod config;
mod ext;
mod handlers;
use crate::handlers::recv;

use self::bot::ONE_BOT;
use self::config::CONFIG;
use arcstr::ArcStr;
use futures::FutureExt;
use mesagisto_client::MesagistoConfig;
use tracing::{info, warn, Level};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use walle_core::app::StandardOneBot;
use walle_core::config::{AppConfig, WebSocketClient};
#[macro_use]
extern crate educe;
#[macro_use]
extern crate automatic_config;
#[macro_use]
extern crate singleton;

#[tokio::main]
async fn main() {
  run().await.unwrap();
}

async fn run() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
          // use local time
          time::UtcOffset::__from_hms_unchecked(8, 0, 0),
          time::macros::format_description!(
            "[year repr:last_two]-[month]-[day] [hour]:[minute]:[second]"
          ),
        )),
    )
    .with(
      tracing_subscriber::filter::Targets::new()
        .with_target("walle-core", Level::DEBUG)
        .with_target("onebot_message_source", Level::INFO)
        .with_target("mesagisto_client", Level::TRACE)
        .with_default(Level::WARN),
    )
    .init();

  if !CONFIG.enable {
    warn!("Mesagisto-Bot is not enabled and is about to exit the program.");
    warn!("To enable it, please modify the configuration file.");
    warn!("Mesagisto-Bot未被启用, 即将退出程序。");
    warn!("若要启用，请修改配置文件。");
    return Ok(());
  }
  MesagistoConfig::builder()
    .name("onebot")
    .cipher_enable(CONFIG.cipher.enable)
    .cipher_key(CONFIG.cipher.key.clone())
    .cipher_refuse_plain(CONFIG.cipher.refuse_plain)
    .nats_address(CONFIG.nats.address.clone())
    .proxy(if CONFIG.proxy.enable {
      Some(CONFIG.proxy.address.clone())
    } else {
      None
    })
    .photo_url_resolver(|id_pair| async { Ok(ArcStr::from("")) }.boxed())
    .build()
    .apply()
    .await;
  let ob = StandardOneBot::new(
    AppConfig {
      websocket: vec![WebSocketClient {
        url: CONFIG.onebot.websocket.to_string(),
        access_token: None,
        reconnect_interval: 4,
      }],
      websocket_rev: vec![],
      ..Default::default()
    },
    handlers::MyHandler,
  )
  .arc();
  ONE_BOT.init(ob.clone());
  recv::recover().await?;
  ONE_BOT.run().await.unwrap();
  tokio::signal::ctrl_c()
    .await
    .expect("无法注册 Ctrl+C 处理回调");
  info!("Mesagisto Bot 正在关闭");
  ONE_BOT.shutdown().await;
  info!("正在保存配置文件");
  CONFIG.save();
  Ok(())
}

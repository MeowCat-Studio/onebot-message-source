#![allow(incomplete_features)]
#![feature(backtrace, capture_disjoint_fields)]

mod bot;
mod config;
mod ext;
mod handlers;
use self::bot::ONE_BOT;
use self::config::CONFIG;
use arcstr::ArcStr;
use futures::FutureExt;
use mesagisto_client::MesagistoConfig;
use tracing::{info, warn};
use walle_core::{AppConfig, WebSocketClient};

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
  let env = tracing_subscriber::EnvFilter::from("warn")
    .add_directive("teloxide=info".parse()?)
    .add_directive("Walle-core=debug".parse()?)
    .add_directive("oms=debug".parse()?)
    .add_directive("onebot_message_source=trace".parse()?)
    .add_directive("mesagisto_client=trace".parse()?);
  tracing_subscriber::fmt().with_env_filter(env).init();

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
  let ob = walle_v11::app::OneBot11::new(
    AppConfig {
      websocket: vec![WebSocketClient {
        url: CONFIG.onebot.websocket.to_string(),
        access_token: None,
        reconnect_interval: 4,
      }],
      websocket_rev: vec![],
      ..Default::default()
    },
    Box::new(handlers::MyHandler),
  )
  .arc();
  ONE_BOT.init(ob.clone());
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

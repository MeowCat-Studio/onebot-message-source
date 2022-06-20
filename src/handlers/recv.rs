use crate::bot::ONE_BOT;
use crate::ext::db::DbExt;
use crate::CONFIG;
use arcstr::ArcStr;
use mesagisto_client::{
  cache::CACHE,
  data::{message::Message, message::MessageType, Packet},
  db::DB,
  server::SERVER,
};
use tracing::trace;
use walle_core::Message as OnebotMessage;
use walle_core::MessageBuild;
use walle_core::{ExtendedMap, MessageSegment};

pub async fn recover() -> anyhow::Result<()> {
  for pair in &CONFIG.bindings {
    SERVER
      .recv(
        ArcStr::from(pair.key().to_string()),
        pair.value(),
        server_msg_handler,
      )
      .await?;
  }
  Ok(())
}
pub async fn add(target: i64, address: &ArcStr) -> anyhow::Result<()> {
  SERVER
    .recv(target.to_string().into(), address, server_msg_handler)
    .await?;
  Ok(())
}
pub async fn change(target: i64, address: &ArcStr) -> anyhow::Result<()> {
  SERVER.unsub(&target.to_string().into());
  add(target, address).await?;
  Ok(())
}
pub async fn del(target: i64) -> anyhow::Result<()> {
  SERVER.unsub(&target.to_string().into());
  Ok(())
}
pub async fn server_msg_handler(
  message: nats::asynk::Message,
  target: ArcStr,
) -> anyhow::Result<()> {
  log::trace!("接收到来自目标{}的消息", target);
  let packet = Packet::from_cbor(&message.data)?;
  match packet {
    either::Left(msg) => {
      left_sub_handler(msg, target).await?;
    }
    either::Right(_) => {}
  }
  Ok(())
}

async fn left_sub_handler(mut message: Message, target: ArcStr) -> anyhow::Result<()> {
  for single in message.chain {
    trace!("正在处理消息链中的元素");
    let sender_name = if message.profile.nick.is_some() {
      message.profile.nick.take().unwrap()
    } else if message.profile.username.is_some() {
      message.profile.username.take().unwrap()
    } else {
      base64_url::encode(&message.profile.id)
    };
    match single {
      MessageType::Text { content } => {

        // BOT.send_group_message(
        //   target.to_string(),OnebotMessage::default().text(content.to_string())
        // ).await?;
      }
      MessageType::Image { id, url } => todo!(),
      MessageType::Edit { content } => todo!(),
    }
  }

  Ok(())
}

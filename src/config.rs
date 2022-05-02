use arcstr::ArcStr;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[basic_derive]
#[derive(AutoConfig)]
#[location = "config/onebot.yml"]
pub struct Config {
  #[educe(Default = false)]
  pub enable: bool,
  // A-z order
  pub bindings: DashMap<i64, ArcStr>,
  pub cipher: CipherConfig,
  pub onebot: OnebotConfig,
  pub nats: NatsConfig,
  pub proxy: ProxyConfig,
}

impl Config {
  pub fn mapper(&self, target: &i64) -> Option<ArcStr> {
    match self.bindings.get(target) {
      Some(v) => return Some(v.clone()),
      None => return None,
    }
  }
}

#[basic_derive]
pub struct NatsConfig {
  // pattern: "nats://{host}:{port}"
  #[educe(Default = "nats://itsusinn.site:4222")]
  pub address: ArcStr,
}

#[basic_derive]
pub struct ProxyConfig {
  #[educe(Default = false)]
  pub enable: bool,
  // pattern: "http://{username}:{password}@{host}:{port}"
  #[educe(Default = "http://127.0.0.1:7890")]
  pub address: ArcStr,
}
#[basic_derive]
pub struct OnebotConfig {
  #[educe(Default = "ws://127.0.0.1:8844")]
  pub websocket: ArcStr,
  #[educe(Default = "")]
  pub access_token: ArcStr,
}

#[basic_derive]
pub struct CipherConfig {
  #[educe(Default = true)]
  pub enable: bool,
  #[educe(Default = "this-is-an-example-key")]
  pub key: ArcStr,
  #[educe(Default = true)]
  pub refuse_plain: bool,
}
